# language: nl

Functionaliteit: Een feature in het Nederlands

Een scenario kan meerdere stappen hebben, en een scenario kan meerdere scenario's hebben.

    Scenario: Scenario 1
        Stel een feit
        Als iets wordt uitgevoerd
        Dan het resultaat is oke

    Scenario: Scenario 2
        Stel een feit
        En een ander feit
        Als iets wordt uitgevoerd
        """
        *Then the result is oke* also some markdown
        """
        Als het resultaat is mislukt