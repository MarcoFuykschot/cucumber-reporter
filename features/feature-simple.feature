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
        """
        *Then the result is oke* also some markdown
        """
        Then the result is failed

    Scenario: Scenario 3 with markdown

        test header 1
        ==

        test header 2
        ---

        markdown test
        - test 1
        - test 2
        <br>
        *Bold text*
        <br>
        ~emphasis text~
        #  will not be rendered as header 1
        ## will not be rendered as header 2
        ```rust
        fn main() {
            println!("Hello, world!");
        }
        ```

        Given a fact
        Then a Skipped line