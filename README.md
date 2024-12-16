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
