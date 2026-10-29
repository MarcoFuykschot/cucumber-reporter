Feature: outline

    Scenario Outline: Scenario Outline name <test>

        Given a fact with '<Header1>'

        Given a fact with '<Header2>'

        Examples:
            | Header1 | Header2 | test       |
            | Value 1 | Value 1 | aaaaaaaaa1 |
            | Value 2 | Value 2 | 2          |
            | Value 3 | Value 3 | 3          |
            | Value 4 | Value 4 | 4          |

    Scenario Outline: Scenario with skipped
        Given a skipped fact with '<Header1>'

        Examples:
            | Header1 | Header2 | test |
            | Value 3 | Value 3 | 3    |
            | Value 4 | Value 4 | 4    |


    Scenario Outline: Scenario with skipped extra new line
        Given a skipped fact with '<Header1>'

        Examples:

            | Header1 | Header2 | test |

            | Value 3 | Value 3 | 3 |

            | Value 5 | Value 4 | 4 |


    Scenario Outline: Scenario with large table
        Given a fact with '<Header1>'

        Examples:
            | Header1 | Header2 | test | a long column name | col2 | a bit longer column name | very long column name | an other very long column name | A other exta long column name to test formating |
            | Value 3 | Value 3 | 3    |                    |      |                          |                       |                                | |
            | Value 5 | Value 4 | 4    |                    |      |                          |                       |                                | |

    Scenario Outline: Scenario with large table no breakable
        Given a fact with '<Header1>'

        Examples:
            | Header1 | Header2 | test | a_long_column_name | col2 | a_bit_longer_column_name | very_long_column_name | an_other_very_long_column_name | A_other_exta_long_column_name_to_test_formating |
            | Value 3 | Value 3 | 3    |                    |      |                          |                       |                                | |
            | Value 5 | Value 4 | 4    |                    |      |                          |                       |                                | |
