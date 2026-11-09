<img width="1919" height="273" alt="Screenshot 2026-11-09 174637" src="https://github.com/user-attachments/assets/47473d62-c08f-4b0b-b9bd-96a05af3b21d" />
Hello, my friend! This is the GitHub repository for my new game, or more precisely a demo, which doesn't have a name yet, so for now it's simply called Project Tactica. It's still a work in progress and there is a lot left to do. Here I want to share what has been done so far and what I'm planning to add.  
<img width="1920" height="1080" alt="Screenshot 2026-11-09 174716" src="https://github.com/user-attachments/assets/9c4fcd17-6836-47e5-81d7-e4bb053d9e7d" />  

# About the demo

This demo consists of one simple playable level. Since the game has tactical controls, let's start with the combat system and overall gameplay.

I classify it as a **tactical turn-based strategy**, something similar to *Mario + Rabbids* or *Persona 5 Tactica*. Those games are my biggest source of inspiration, even though my project differs from them quite a lot in mechanics and game flow.

You can either download a compiled build from the [Releases](#) page, or take the source code and compile it yourself.

# Gameplay

The whole gameplay is built around a few ideas:

1. **The goal is to get from point A to point B, not to eliminate every enemy.** You can technically be a pacifist and finish the level without a single neutralization. I don't want to limit the player to one specific play style.
2. **There is no HP, there is an awareness level.** The player's awareness rises when he neutralizes someone or simply stays near an enemy, and it decreases when he is away from enemies. If it gets too high, it's game over, so you have to keep it under control.
3. **You don't wait for enemy turns.** Enemies take their turns at the same moment as the player. The game flow is still divided into turns (or steps, call it what you like), but everything happens simultaneously.
4. **Actions are few, clear and simple.** The full list is shown in the screenshot above. The game is deliberately not overcomplicated.

I think this is enough to understand the game flow.

# Technical details

# Engine

The game runs on my own engine, **schnellwerke**, currently at version 4.0 and still under development. The purpose of this demo is to finalize the native version of the engine; the web version will come later.

# Rendering

The demo uses all the new engine features, especially the new lighting system. There are two render paths:

- **Software ray tracing (RT) shader.** The lighting was designed with this path in mind, and the game looks better with it.
- **Classic PBR rasterization.** Still available, intended for weaker devices.

GPU requirements aren't very high. The demo runs at about **30 FPS at 0.5 resolution scale at 1080p** on the integrated Vega 7 of a Ryzen 5600G. That isn't a very impressive result, but I think the vast majority of dedicated GPUs will outperform it, since the software RT path depends heavily on memory bandwidth and raw compute. Vega 7 is roughly comparable to the PS4 or Steam Deck class (the Deck has faster memory).

# Requirements

- **CPU with AVX support.** The compiled build requires it, since AVX gives a huge performance boost (around 3x-6x on the CPU side).
- If your CPU doesn't support AVX, you can compile the demo from source for your target CPU.
- Targeting **AVX2** or **AVX512** may give even more performance through accelerated matrix multiplication.

# Art style

The game is heavily stylized, with a comic look inspired by *Persona*, *XIII* and a bit of *Max Payne*.

At the moment there is still a lot to do: no characters are drawn and no story comics exist yet. I wanted to start with the code and gameplay side first: mechanics, menus and so on. Even so, there are already a few things worth looking at:

- **UI styled as comic panels and speech bubbles**, often decorated with animations.
- **Locations follow the same logic**: simple materials, only a small number of complex textures, and atmosphere created mostly through geometric complexity and lighting.

# Story

As with my other projects, the setting is based on real events, and here it's my favourite period: the **Cold War, October 1973**, one of its most tense moments, comparable to the Cuban Missile Crisis. It's just hours before the Yom Kippur War begins.

- You play as a West German operative with the codename **"Junge"** (he's the youngest operative in the service).
- West German intelligence already knows that East Germany is shipping military aid, but not to whom. They waited for Soviet military advisors to visit East Germany, and now Junge has to get into East Berlin, to the place where East German and Soviet officials meet, and gather intel on those shipments.
- The story is short, just like the final game, and takes place over a few hours on the eve of the Yom Kippur War.

# Builds

# Build 1

This is the first public build. Here is what works in it:

- **Game logic is basically done.** You can complete the demo without any issues.
- **The first level is done.**
- **Most of the UI is done and functional.**
- **Settings are saved without problems.** The demo is too short to have savepoints, so there are no saves.
- **Controls:** PC keyboard and mouse, plus complete gamepad support.
- **Full Russian translation.** For now it mostly serves as a test of the Cyrillic integration.
- **Rendering looks finished.** I expect it to stay mostly unmodified in the future.

# Status

This demo is still in development. A lot has been done and a lot more is still to come, so stay tuned: I'll share updates and new builds here.
