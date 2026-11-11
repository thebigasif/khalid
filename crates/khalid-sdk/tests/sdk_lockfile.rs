//! `sdk/typescript/package-lock.json` pins what `package.json` asks for
//! (gap audit 4 row 44, G27). CI runs `npm ci` in the SDK folder, and
//! `npm ci` refuses a lockfile that has drifted from `package.json`. This
//! test says so before a push, on a machine without npm.

use std::fs;
use std::path::PathBuf;

use serde_json::Value;

/// One file in the SDK folder, read as JSON.
#[cfg(test)]
fn sdk_json(name: &str) -> Value {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("../../sdk/typescript");
    path.push(name);
    let text = fs::read_to_string(&path).unwrap();
    serde_json::from_str(&text).unwrap()
}

#[test]
fn the_lockfile_root_matches_package_json() {
    let package = sdk_json("package.json");
    let lock = sdk_json("package-lock.json");
    assert_eq!(lock["lockfileVersion"], 3);
    assert_eq!(lock["name"], package["name"]);
    assert_eq!(
        lock["version"], package["version"],
        "bump the lockfile with package.json"
    );
    let root = &lock["packages"][""];
    for field in ["name", "version", "license", "engines", "devDependencies"] {
        assert_eq!(
            root[field], package[field],
            "packages[\"\"].{field} differs"
        );
    }
    assert!(
        package.get("dependencies").is_none(),
        "the SDK has no runtime dependencies"
    );
}

#[test]
fn every_dev_dependency_is_locked_at_its_exact_version() {
    let package = sdk_json("package.json");
    let lock = sdk_json("package-lock.json");
    let wanted = package["devDependencies"].as_object().unwrap();
    for (name, version) in wanted {
        let entry = &lock["packages"][format!("node_modules/{name}")];
        assert_eq!(
            &entry["version"], version,
            "{name} is locked at another version"
        );
    }
}

#[test]
fn every_locked_package_has_a_registry_tarball_and_a_sha512() {
    let lock = sdk_json("package-lock.json");
    let packages = lock["packages"].as_object().unwrap();
    for (path, entry) in packages.iter().filter(|(path, _)| !path.is_empty()) {
        let name = path.strip_prefix("node_modules/").unwrap();
        let version = entry["version"].as_str().unwrap();
        let file = name.rsplit('/').next().unwrap();
        let tarball = format!("https://registry.npmjs.org/{name}/-/{file}-{version}.tgz");
        assert_eq!(entry["resolved"].as_str(), Some(tarball.as_str()), "{path}");
        let integrity = entry["integrity"].as_str().unwrap();
        let hash = integrity.strip_prefix("sha512-").unwrap();
        assert_eq!(hash.len(), 88, "{path}: a sha512 is 88 base64 characters");
        assert_eq!(
            entry["dev"], true,
            "{path}: the SDK installs only dev tools"
        );
    }
}

#[test]
fn every_locked_dependency_is_itself_locked() {
    let lock = sdk_json("package-lock.json");
    let packages = lock["packages"].as_object().unwrap();
    for (path, entry) in packages {
        for field in ["dependencies", "devDependencies", "optionalDependencies"] {
            let Some(needs) = entry[field].as_object() else {
                continue;
            };
            for name in needs.keys() {
                let key = format!("node_modules/{name}");
                assert!(
                    packages.contains_key(&key),
                    "{path} needs {name}, which is not locked"
                );
            }
        }
    }
}
