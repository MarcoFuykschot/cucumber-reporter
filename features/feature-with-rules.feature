Feature: rules

    Rule: rule 1
        Description of a rule

        Scenario: Scenario 1
            Given a fact
            When something is executed
            Then the result is oke

        Scenario: Scenario 2
        Description test scenario
            Given a fact
            And a other fact
            When something is executed
            Then the result is failed

    Rule: rule 2

        Scenario: Scenario 1
            Given a fact
            And a other fact
            When something is executed
            Then the result is failed