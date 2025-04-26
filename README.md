# ML~S~A (Mini LLM Sla... Agents)

## Intro

This is an ambious project that I thought of at the night before sleeping to put up a bunch of LLMs at work for programming the needs of my project while i focus on other things.
This gives a real projectie vibe on github since unlike IDEs, I'll be reviewing the code itself on github and not interact with the agents directly. Only via github.

Why? This kinda helps me learn how to review code properly and also I can program from my phone this way too.

## How does this work?

The idea behind this is to have 4 LLM slaves/workers plan and work together.
The first worker is called the Planner. Its job is just to plan the steps for the code and make a PR.
The second worker is called the Coder. Fairly obvious what the mad lad will do.
Then comes our tester who will test the code and make sure everything is in order before making the PR.

Once the PR is made, I'll review it and give my insights on which the planner can argue or work on it depending on if I am wrong or not. If I am not, then we have changes coming in and if I am wrong then the PR gets merged in.


Simple.
