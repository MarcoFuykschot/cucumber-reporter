Feature: simple
    With a description same Feature as in parent folder

    Scenario: Scenario 1 test in subfolder
        Given a fact
        When something is executed
        Then the result is oke

    Scenario: Scenario added in subfolder
        Given a fact
        And a other fact
        When something is executed
        Then the result is failed

     Scenario: Scenario outline 3
        Given a fact
        Then a Skipped line