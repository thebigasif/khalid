---
title: "Gear testing at Bench Test: how we measure every product"
slug: gear-testing
date: 2026-06-16
author: kit-adebayo
reviewed_by: oskar-brede
category: guides
description: How our gear testing works, test by test, for microphones, cameras, lenses, lights, interfaces and headphones, and how it becomes a score out of 10.
focus_keyword: gear testing
tags: [inside bench test, methods]
image:
  src: lead.jpg
  alt: A photo studio set up for gear testing, with a white paper backdrop lit by two large octagonal softboxes on stands and a ring of lights hanging from a ceiling rail above
---

Our gear testing puts every product of a kind through the same tests, on the same bench, with the same tools, and then a reviewer uses it for at least two weeks of real work. The tests give us figures you can compare across reviews, and the weeks of use tell us what the figures miss.

Every "Tested" table on this site comes from the tests below. When we change a test, we say so here and in each review it affects.

::toc

## The gear testing bench

The lab is one room of 4 by 5 metres. In one corner stands a small booth lined with 10 cm of mineral wool, where a voice dies away in about a fifth of a second. The rest of the room is left bare on purpose, with a hard floor and a large window, so we can hear how gear behaves in the kind of room most of our readers have.

Our main tools are:

- **A calibrated reference microphone and sound level meter**, for noise and loudness.
- **A light meter**, checked against a reference lamp every three months.
- **A printed colour chart** with 24 patches, filmed under every light and through every camera.
- **An autofocus course**: a marked path on the floor that a presenter walks, turns and walks back along.
- **A set of test files**: tones, sweeps and spoken passages, played through every interface and pair of headphones.

## Microphones: one script, two rooms

Each microphone reads the same 90-second script twice: in the booth and in the bare room. The reader sits 10 cm from the microphone, then 20 cm. We measure:

| Test | How we run it | What it tells you |
|---|---|---|
| Gain for a quiet voice | Gain needed for a soft reader to peak at minus 12 | Whether your interface can drive it |
| Self-noise | Measured in the booth with the capsule covered | Hiss you will hear in pauses |
| Room pickup | Panel rating of the bare-room recording, 1 to 10 | How much echo you will hear |
| Plosives | 20 words starting with "p", counted pops | Whether you need a pop filter |
| Off-axis rejection | Level drop for a voice directly behind | How much of a second host it hears |

Three listeners then rate the voice sound blind, without knowing which microphone they hear. The voice sound and the room pickup make up half of a microphone's score. The [Wren D7 review](/blog/wren-d7-review/) shows these figures in use.

## Cameras and lenses: the autofocus course

Cameras walk our autofocus course in 4K. A presenter walks towards the camera, turns to look at a shelf, and walks back. We count the frames where the eyes are sharp. We then record 4K at 30 frames a second until the camera stops, by heat, battery or a full card, and note which came first.

Lenses are tested on one camera body. We film a flat test chart at the widest aperture and two stops down, and score the centre and corners against the best lens we have measured on that body. We record the lens focusing near to far with the camera's microphone, to hear the motor, and measure how much the framing changes.

## Lights: lux, colour and fan noise

Each light goes on a stand one metre from the meter, bare and then through the softbox we use for every light. We report lux at one metre, so you can compare lights of different sizes fairly. We film our colour chart under the light and calculate the TLCI, a score of colour accuracy for video where 100 is perfect. We measure fan noise at one metre in each fan mode and film at a fast shutter speed with the dimmer near its lowest setting, to catch flicker.

## Audio gear: test files and a blind panel

Interfaces and recorders get our test files and a noise measurement at fixed gain settings. We report noise as EIN, the noise of the preamp itself, where a lower number is better and about -128 dBu is the limit. We also time how long each one takes to start and count the steps to the settings people use most.

Headphones are tested on a measurement rig and by the panel, who listen for how evenly they play low, middle and high sounds. For headphones meant for recording, we also check how much sound leaks out, since a leaking pair can be picked up by the microphone.

## How gear testing becomes a score

Every product gets a score out of 10. The weights are the same for each kind of product:

| Part of the score | Weight | Where it comes from |
|---|---|---|
| Sound or image quality | 50% | Measured figures and the blind panel |
| Ease of use | 20% | Two weeks of real use |
| Build and reliability | 15% | Use, plus anything that broke or overheated |
| Value for money | 15% | Price against the products we have tested |

A score of 8 or more means we would buy the product ourselves at its price. A score under 6 means we would not recommend it.

:::callout{kind=note}
We buy most products ourselves. When a maker lends us one, the review says so and we send it back afterwards. Makers never see a review before it goes live.
:::

## When we test again

We repeat our gear testing when a firmware update changes how a product behaves, or when a reader or a maker questions a result and sends us their evidence. The [Pike X2 camera review](/blog/pike-x2-review/) is an example: firmware 1.3 fixed the overheating we found at launch, so we ran the recording test again and raised the score. The review shows both results and the date of each.

If you think one of our figures is wrong, write to lab@example.com with what you measured and how. The [team behind Bench Test](/blog/about/) is on our about page. For background on why we measure light in lux, see the article on [lux](https://en.wikipedia.org/wiki/Lux).
