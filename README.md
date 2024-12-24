# Auto-Gippity

This is a walk-through of the [Build an AutoGPT Code Writing AI Tool With Rust and GPT-4](https://www.udemy.com/course/autogpt-gpt4-code-writing-ai) Udemy course.

This project consists of three agents, performing different tasks:

1. Managing agent - manages a solutions' architect
2. Solutions' architect
3. Backend Developer

Agents can call LLMs (Large Language Models) and get back some results.

This program takes user input from command line.

## Agents

All agents have an underlying `BasicAgent` that describes their behaviour. Managing agent holds
_fact sheets_ - information that agents need in order to process the task at hand.

Please check the diagram below to see the relationship between three agents:

![Agents diagram](images/agents_relationship.png)

## Fact sheet

This holds the following information:

|        Fact         |        Owner        |
| :-----------------: | :-----------------: |
| Project description |   Project manager   |
|    Project scope    | Solutions architect |
|    External urls    | Solutions architect |
|    Backedn code     |  Backend developer  |
| API Endpoint schema |  Backend developer  |

## Runbook

Currently you will have to have `rust` installed and run `cargo run` from the root of the project.

When prompted, describe what is the website that you would like Auto-Gippity to build:

![prompt example](images/prompt_be_like.png)

The result should then be a functional application that:

- compiles
- passes functional testing

Here is a glimpse of what the generated code looks like:

![result](images/result.png)

Code is currently hard coded to be saved into a local (siebling) repository named `web_template_autogpt`,
this can, again, be further improved in the future to create a new repository, push to it, compile and
publish to AWS CodeArtifact (or wherever), and actually deploy into a cloud.
