Feature: simple
    With a description

    Scenario: Scenario 1
        Given a fact
        When something is executed
        Then the result is oke

    Scenario: Scenario 2
        Given a fact
        And a other fact
        When something is executed
        Then the result is failed

    Scenario: Scenario 3 with markdown

        test header 1
        -------------------
        markdown test
        - test 1
        - test 2

        Given a fact
        Then a Skipped line