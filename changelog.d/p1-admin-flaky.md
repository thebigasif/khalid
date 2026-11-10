Fixed
- The admin's test run no longer fails now and then with "window is not defined" after every test passed. The start-a-post tests ended while the editor was still loading; they now wait for it, and every admin test waits for any page it opened to finish loading.
