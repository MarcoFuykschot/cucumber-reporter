@tag1 @tag2
Feature: Feature with tags

    Feature Description

    # comment
    Scenario Outline: Scenario Outline name with tags <test>

        Given a fact with '<Header1>'
        Given a fact with '<Header2>'

        Examples:
            | Header1 | Header2 | test           |
            | Value 1 | Value 1 | aaaaaaaa1 asdf |
            | Value 2 | Value 2 | aaa     aaaaa2 |
            | Value 3 | Value 3 | aaaaaaaa3      |
            | Value 4 | Value 4 |                |