Feature: outline with background

    Background: Given some background

        Given a fact with 'test-background-1'

        Given a fact with 'test-background-2'



    Scenario Outline: Scenario Outline name <test>



        Given a fact with '<Header1>'

        Given a fact with '<Header2>'

        Examples:
            | Header1 | Header2 | test |
            | Value 1 | Value 1 | aaaaaaaaaaaaaaaaaaaaaaaaaaa1    |
            | Value 2 | Value 2 | 2    |
            | Value 3 | Value 3 | 3    |
            | Value 4 | Value 4 | 4    |

    Scenario Outline: Scenario with skipped
        Given a skipped fact with '<Header1>'

        Examples:
            | Header1 | Header2 | test |
            | Value 3 | Value 3 | 3    |
            | Value 4 | Value 4 | 4    |


