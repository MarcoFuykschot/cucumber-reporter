

## Examples

A simple gherkin feature file

<!--CONTENT-START:features/simple-feature.feature:Feature-->
```Feature
Feature: Simple feature
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
        
     Scenario: Scenario 3
        Given a fact
        Then a Skipped line
```
<!--CONTENT-END:features/simple-feature.feature-->

Will produce the following html output

![simple](/assets/simple.png)

A gherkin feature file using and outline 

<!--CONTENT-START:features/feature-with-outline.feature:Feature-->
```Feature
Feature: documentation with outline

    Scenario Outline: Scenario Outline name <test>
        Given a fact with '<Header1>'
        Given a fact with '<Header2>'

        Examples:
            | Header1 | Header2 | test |
            | Value 1 | Value 1 | 1    |
            | Value 2 | Value 2 | 2    |
            | Value 3 | Value 3 | 3    |
            | Value 4 | Value 4 | 4    |

     Scenario Outline: Scenario with skipped
        Given a skipped fact with '<Header1>'

        Examples:
            | Header1 | Header2 | test |
            | Value 3 | Value 3 | 3    |
            | Value 4 | Value 4 | 4    |
   
```
<!--CONTENT-END:features/feature-with-outline.feature-->

![simple](/assets/outline.png)
