## Goal

To create a reporter for the crate cucumber, that directly
produces html files. 

for each execute feature an html file is produced using the 
name of the feature. At the end an index file is produced
with all executed features and some stats.

You can use the commandline option --output-html-path to change the output
path of the html files, default it uses the current directory where the tests
are run.

## Examples

### A simple gherkin feature file

<!--CONTENT-START:features/simple-feature.feature:Feature-->
```Feature
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
        
     Scenario: Scenario 3
        Given a fact
        Then a Skipped line
```
<!--CONTENT-END:features/simple-feature.feature-->

Will produce the following [html output](assets/simple.html)

### A gherkin feature using an outline

<!--CONTENT-START:features/feature-with-outline.feature:Feature-->
```Feature
Feature: outline

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

Will produce the following output


## planned 

* direct pdf output
* custom templating  custom templating 