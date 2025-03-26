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

<!--CONTENT-START:assets/Simple feature.html:-->
<html><head><style>.title{color:#004080;border-bottom:1px solid #ccc;padding-bottom:5px}.desc{margin:10px 0;line-height:1.5}.results{border-collapse:collapse;width:100%;margin:2px 0}.row{text-align:left;border:1px solid #ddd;padding:2px}.heading{background-color:#f2f2f2}.Failed{color:#b22222}.Passed{color:#006400}.NotRun{color:#696969;font-style:italic}.bg_Failed{background:radial-gradient(circle,#fff 75%,red 100%);font-weight:bolder}.bg_Passed{color:#006400}.bg_NotRun{color:#696969;font-style:italic}</style></head><body><div><h1 class=title>Simple feature</h1><p class=desc>With a description</p><h3 class=title>Scenario 1</h3><p class=description></p><table class=results><thead><tr class="row heading"><th>Step</th><th>Outcome</th></tr></thead><tbody><tr class="row Passed"><td>Given a fact</td><td>Passed</td></tr><tr class="row Passed"><td>When something is executed</td><td>Passed</td></tr><tr class="row Passed"><td>Then the result is oke</td><td>Passed</td></tr></tbody></table><hr><h3 class=title>Scenario 2</h3><p class=description></p><table class=results><thead><tr class="row heading"><th>Step</th><th>Outcome</th></tr></thead><tbody><tr class="row Passed"><td>Given a fact</td><td>Passed</td></tr><tr class="row Passed"><td><span style=margin-left:10px> And a other fact </span></td><td>Passed</td></tr><tr class="row Passed"><td>When something is executed</td><td>Passed</td></tr><tr class="row Failed"><td>Then the result is failed</td><td>Failed</td></tr></tbody></table><hr><h3 class=title>Scenario 3</h3><p class=description></p><table class=results><thead><tr class="row heading"><th>Step</th><th>Outcome</th></tr></thead><tbody><tr class="row Passed"><td>Given a fact</td><td>Passed</td></tr><tr class="row NotRun"><td>Then a Skipped line</td><td>NotRun</td></tr></tbody></table><hr></div></body></html>
<!--CONTENT-END:assets/Simple feature.html--> 

### A gherkin feature using an outline

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

Will produce the following output

<!--CONTENT-START:assets/documentation with outline.html:-->
<html><head><style>.title{color:#004080;border-bottom:1px solid #ccc;padding-bottom:5px}.desc{margin:10px 0;line-height:1.5}.results{border-collapse:collapse;width:100%;margin:2px 0}.row{text-align:left;border:1px solid #ddd;padding:2px}.heading{background-color:#f2f2f2}.Failed{color:#b22222}.Passed{color:#006400}.NotRun{color:#696969;font-style:italic}.bg_Failed{background:radial-gradient(circle,#fff 75%,red 100%);font-weight:bolder}.bg_Passed{color:#006400}.bg_NotRun{color:#696969;font-style:italic}</style></head><body><div><h1 class=title>documentation with outline</h1><p class=desc></p><h3 class=title>Scenario Outline name &lt;test></h3><p class=desc></p><table class=results><thead><tr class="row heading"><th>Step</th></tr></thead><tbody><tr class=row><td>Given a fact with '&lt;Header1>'</td></tr><tr class=row><td>Given a fact with '&lt;Header2>'</td></tr></tbody></table><h4>Example</h4><p></p><table class=results><thead><th class="row heading">Header1</th><th class="row heading">Header2</th><th class="row heading">test</th><th class="row heading">Outcome</th></thead><tbody><tr class="row bg_Passed"><td style=text-align:center>Value 1</td><td style=text-align:center>Value 1</td><td style=text-align:center>1</td><td><table class=results><thead><tr class="row heading"><th>Step</th><th>Outcome</th></tr></thead><tbody><tr class="row Passed"><td>Given a fact with 'Value 1'</td><td>Passed</td></tr><tr class="row Passed"><td>Given a fact with 'Value 1'</td><td>Passed</td></tr></tbody></table></td></tr><tr class="row bg_Failed"><td style=text-align:center>Value 2</td><td style=text-align:center>Value 2</td><td style=text-align:center>2</td><td><table class=results><thead><tr class="row heading"><th>Step</th><th>Outcome</th></tr></thead><tbody><tr class="row Failed"><td>Given a fact with 'Value 2'</td><td>Failed</td></tr><tr class="row NotRun"><td>Given a fact with 'Value 2'</td><td>NotRun</td></tr></tbody></table></td></tr><tr class="row bg_Passed"><td style=text-align:center>Value 3</td><td style=text-align:center>Value 3</td><td style=text-align:center>3</td><td><table class=results><thead><tr class="row heading"><th>Step</th><th>Outcome</th></tr></thead><tbody><tr class="row Passed"><td>Given a fact with 'Value 3'</td><td>Passed</td></tr><tr class="row Passed"><td>Given a fact with 'Value 3'</td><td>Passed</td></tr></tbody></table></td></tr><tr class="row bg_Passed"><td style=text-align:center>Value 4</td><td style=text-align:center>Value 4</td><td style=text-align:center>4</td><td><table class=results><thead><tr class="row heading"><th>Step</th><th>Outcome</th></tr></thead><tbody><tr class="row Passed"><td>Given a fact with 'Value 4'</td><td>Passed</td></tr><tr class="row Passed"><td>Given a fact with 'Value 4'</td><td>Passed</td></tr></tbody></table></td></tr></tbody></table><h3 class=title>Scenario with skipped</h3><p class=desc></p><table class=results><thead><tr class="row heading"><th>Step</th></tr></thead><tbody><tr class=row><td>Given a skipped fact with '&lt;Header1>'</td></tr></tbody></table><h4>Example</h4><p></p><table class=results><thead><th class="row heading">Header1</th><th class="row heading">Header2</th><th class="row heading">test</th><th class="row heading">Outcome</th></thead><tbody><tr class="row bg_NotRun"><td style=text-align:center>Value 3</td><td style=text-align:center>Value 3</td><td style=text-align:center>3</td><td><table class=results><thead><tr class="row heading"><th>Step</th><th>Outcome</th></tr></thead><tbody><tr class="row NotRun"><td>Given a skipped fact with 'Value 3'</td><td>NotRun</td></tr></tbody></table></td></tr><tr class="row bg_NotRun"><td style=text-align:center>Value 4</td><td style=text-align:center>Value 4</td><td style=text-align:center>4</td><td><table class=results><thead><tr class="row heading"><th>Step</th><th>Outcome</th></tr></thead><tbody><tr class="row NotRun"><td>Given a skipped fact with 'Value 4'</td><td>NotRun</td></tr></tbody></table></td></tr></tbody></table></div></body></html>
<!--CONTENT-END:assets/documentation with outline.html-->

## planned 

* direct pdf output
* custom templating 