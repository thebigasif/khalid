---
title: Core Web Vitals einfach erklärt für Texter
slug: core-web-vitals
date: 2026-10-01
reviewed: 2026-10-15
author: tomas-lindqvist
reviewed_by: priya-raman
category: technical-seo
description: Was die Core Web Vitals messen, welche Werte als gut gelten und welche Gewohnheiten Ihre Beiträge schnell halten, ganz ohne Hilfe aus der Technik.
translation: core-web-vitals
focus_keyword: Core Web Vitals
tags: [ladezeit, bilder]
image:
  src: cover.svg
  alt: Ein Tachometer mit der Nadel im grünen Bereich über dem Ladebalken einer Seite
---

Core Web Vitals sind drei Messwerte, mit denen Google beurteilt, wie sich eine Seite für Leser anfühlt. Wie schnell erscheint der Hauptinhalt? Wie schnell reagiert die Seite auf einen Klick? Und springt beim Laden etwas hin und her? Texter schreiben selten Code. Trotzdem stammt vieles, was einen Beitrag langsam macht, aus der Redaktion.

Dieser Beitrag erklärt die drei Werte in einfachen Worten. Er zeigt, welche Ergebnisse als gut gelten. Und er nennt die Gewohnheiten, mit denen Ihre Beiträge schnell bleiben.

## Was die drei Core Web Vitals messen

Jeder Wert beantwortet eine Frage, die ein Leser stellen würde.

- **Erscheint die Seite bald?** Das misst der [[largest-contentful-paint|Largest Contentful Paint]]. Er zeigt den Moment, in dem das größte Bild oder der größte Textblock sichtbar ist.
- **Reagiert die Seite, wenn ich tippe?** Das misst Interaction to Next Paint, kurz INP. Es ist die Zeit zwischen einem Klick und der nächsten Änderung auf dem Bildschirm.
- **Bleibt alles an seinem Platz?** Das misst Cumulative Layout Shift, kurz CLS. Der Wert zählt zusammen, wie weit sich Dinge beim Laden verschieben.

:::figure{src="lcp.svg" alt="Drei Phasen beim Laden einer Seite, wie sie die Core Web Vitals messen"}
Der Largest Contentful Paint ist das dritte Bild: der Moment, in dem das größte Element angekommen ist.
:::

## Welche Werte als gut gelten

Google teilt jeden Wert in gut, verbesserungswürdig und schlecht ein. Eine Seite besteht, wenn die meisten echten Besuche gut abschneiden. Die Grenzen stammen aus dem [Leitfaden zu Web Vitals auf web.dev](https://web.dev/articles/vitals).

| Wert | Was er misst | Guter Wert |
|---|---|---|
| LCP | Das größte Element erscheint | Im Jahr 2026 höchstens 2,5 Sekunden |
| INP | Die Seite reagiert auf Eingaben | Im Jahr 2026 höchstens 200 Millisekunden |
| CLS | Das Layout bleibt ruhig | Im Jahr 2026 höchstens 0,1 |

Die Werte stammen aus echten Besuchen, nicht aus einem Test auf Ihrem Laptop. Google nimmt den Wert, den drei von vier Besuchen erreichen. Das langsamste Viertel Ihrer Leser entscheidet also über das Ergebnis.

:::callout{kind=note}
Im März 2024 hat INP den alten Wert First Input Delay abgelöst. Ältere Ratgeber, die noch von First Input Delay sprechen, sind veraltet.
:::

## Warum Texter die Werte stärker prägen als gedacht

Ein Entwickler baut die Vorlage einmal. Die Core Web Vitals ändern sich aber mit jedem neuen Beitrag. Die Redaktion fügt jede Woche Neues hinzu: ein Titelbild, eine Grafik, ein Video, einen Beitrag aus einem sozialen Netzwerk. Für jedes Teil muss der Browser eine Datei laden.

Das größte Teil ist fast immer das Titelbild. Es ist meist das größte Element auf dem Bildschirm. Seine Größe entscheidet deshalb über den Largest Contentful Paint. Ein Foto direkt aus der Kamera ist oft zehnmal schwerer als nötig.

:::callout{kind=warning}
Laden Sie nie ein Foto direkt vom Handy hoch. Verkleinern Sie es auf die Breite, die Ihr Blog zeigt, und komprimieren Sie es danach.
:::

## Gewohnheiten für schnelle Beiträge

Diese Regeln für gute Core Web Vitals gelten bei Larkspur für jeden Beitrag. Keine davon braucht einen Entwickler.

1. **Bilder vor dem Hochladen verkleinern.** Nicht breiter als die breiteste Ansicht Ihres Blogs.
2. **Das passende Format wählen.** Fotos als JPEG oder WebP, Zeichnungen als SVG.
3. **Jedes Bild beschreiben.** Ein Alternativtext kostet keine Zeit und hilft allen, die das Bild nicht sehen.
4. **Videos sparsam einbetten.** Ein Video pro Beitrag ist gut. Vier Videos sind zu viel.
5. **Den Anfang leicht halten.** Oben stehen Text und ein Bild, kein Bilderkarussell.

## Der Vortrag zur Reaktionszeit

Das Chrome-Team zeigt in diesem Vortrag aus dem Jahr 2024, wie man langsame Klicks hinter einem schlechten INP findet. Der Vortrag ist auf Englisch.

::embed{src="https://www.youtube.com/watch?v=xfjumh8ySRY" title="New field insights for debugging INP"}

## So prüfen Sie Ihre eigenen Seiten

Die Search Console hat einen eigenen Bericht für die Core Web Vitals. Er sortiert Ihre Seiten nach den Werten echter Besuche. Schauen Sie einmal im Monat hinein, zuerst bei den Beiträgen mit den meisten Besuchern.

Gute Core Web Vitals zahlen sich auch an anderer Stelle aus. Wer einen Beitrag plant, sollte zuerst [verstehen, was Suchende wirklich wollen](/blog/de/suchintention/). Und wer von KI-Antworten zitiert werden will, braucht Seiten, die schnell und stabil laden. Mehr dazu in unserem Beitrag, [wie KI-Suchdienste ihre Quellen auswählen](/blog/de/ki-suche-quellen/).
