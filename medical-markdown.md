# Medical Markdown

For a general explanation of what Medical Markdown actually is and its main concepts, see the [README.md][] at the top level of the Medical Markdown repository.

General parsing rules:
* Headings and Subheadings are delineated by a small number of characters, followed by a forward-slash character
* Subsequent lines are then regarded as structured under that heading. Depending on the heading type, the content may either be paragraph text, a bulleted list, ordered list, or other form of data. See the individual data items for specifics.
* A blank line marks the end of the section. Ignore non-printing characters such as spaces or tabs.

Guide to syntax
* anything in backticks '`'' is something the user types in
* the rocket '=>'' indicates the resultant output from the parser 


## Core Clinical Headings ##

### `PC/` ###
* Heading: "Presenting complaints or issues"
* Clinical description: The list and description of the health problems and issues experienced by the patient resulting in their attendance. These may include disease state, medical condition, response and reactions to therapies. Eg blackout, dizziness, chest pain,follow up from admission, falls, a specific procedure, investigation or treatment.

### `HPC/` ###
History of the Presenting Complaint(s)


### `FH/` ###
* Heading: "Family history"
* Clinical description: The record of relevant illness in family relations deemed to be significant to the care or health of the patient, including mental illness and suicide, genetic information etc.



## Examination ##

The section tag `OE/` demarcates an 'On Examination' section. Within an on examination section, you can use the following tags to create structured data:

`PR/` pulse rate
example: `PR/ 87 reg` => 


`BP/` blood pressure 


`GCS/` Glasgow Coma Scale (Assessment Scale)
this score can be further broken down into its consituent parts for Best Eye, Verbal, and Motor responses:
example: `GCS/ 15` => {"glasgow-coma-scale": "15"}


`AVPU/` Alert - Voice - Pain - Unresponsive
example `AVPU/ A` => {"avpu":"alert"}

`RS/` Respiratory System
* free text or coded data SNOMED (finding)

`CVS/` Cardiovascular System
`HS/` heart sounds
* free text or coded data SNOMED (finding)
* `1+2+0` signifies `1st and 2nd heart sound heard, no additional sounds` and could be represented in appropriate SNOMED (finding) codes

PNS/ Peripheral Nervous System


CNS/ Central Nervous System

IMP/ Impression (this is used often in place of Diagnosis as it is less categorical)

PLAN/ Plan

TODO/ A todo or jobs list
* bulleted list
* assign via @
* context via #
* (use todo.txt spec)




[README.md]: 


