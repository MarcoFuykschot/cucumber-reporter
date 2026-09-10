Feature: background without rules

    Background: Scenario Background
        Given a fact
        When something is executed
        Then the result is oke

    Scenario: Scenario 2
        Description test scenario
        Given a fact
        And a other fact
        When something is executed
        Then the result is failed

    Scenario: Scenario 1
        Given a fact
        And a other fact
        When something is executed
        Then the result is failed