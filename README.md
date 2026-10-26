# Determination Labs Documents

This repository hosts Determination Labs Documents (DLDs). Each document lives in its own directory named `dld-NNNN`, where NNNN is the document number zero-padded to four digits. The document itself is the `README.md` in that directory, and its supporting material sits alongside it. Numbers are assigned in sequence when the directory is created and are never reused, including for withdrawn documents.

# Document Format

A document is made up of the following parts, in this order.

1. **Title**: a top-level heading with the document's title.
2. **Preamble**: the metadata of the document, described below.
3. **Abstract**: a single paragraph summarising the document.
4. **Body**: the substance of the document, organised into sections suited to its subject. In a Specification, the Body opens with the statement that the key words of RFC 2119 are to be interpreted as described there, then the normative sections. In an Informational document, the Body consists of explanatory sections and does not use the key words.
5. **Implementations**: a list of implementations, each beginning with its language in bold, followed by a link to its source repository and, where it is published, its package.
6. **References**: the external sources the document links to, in APA style.

## Preamble

A document's title is followed by the preamble. The preamble is a fenced block of `Name: Value` lines, one field per line, in the order shown, with the values aligned in a column. Conditional fields are omitted when they do not apply.

```
DLD:            <number>
Type:           <type>
Status:         <status>
Authors:        <surname>, <given names>
Created:        <YYYY-MM-DD>
Updated:        <YYYY-MM-DD>
Requires:       <numbers>
Supersedes:     <number>
Superseded-By:  <number>
```
### DLD

The **DLD** field is the document number, without padding.

### Type

The **Type** field is either `Specification` or `Informational`. Only Specifications are normative, and they use the key words of RFC 2119.

### Status

The **Status** field indicates one of the following:

- **In Progress**: the first version of a document currently being written. The document may be incomplete and may change in any way, including being abandoned. It is not yet ready for review.
- **Draft**: the first version is complete and open to review and revision. Normative changes are still expected and allowed.
- **Final**: the authors consider the document complete. A Specification additionally has test vectors in its directory and at least one implementation shown to pass them. Only errata and non-normative clarifications may change it, and any other change requires a new document that supersedes it.
- **Deprecated**: a Final document whose use is discouraged without a replacement, for example after a flaw is found in it.
- **Superseded**: a Final or Deprecated document replaced by a newer one, named in `Superseded-By`.
- **Withdrawn**: a document abandoned by its authors before reaching Final. It is not to be implemented.

The permitted transitions are In Progress to Draft, Draft to Final, Final to Deprecated or Superseded, Deprecated to Superseded, and In Progress or Draft to Withdrawn.

### Authors

The **Authors** field lists one or more author names, each as the surname followed by the initials of the given names (Surname, Initials), separated by semicolons.

### Created

The **Created** field is the date the document was added, as an ISO 8601 extended format calendar date (YYYY-MM-DD).

### Updated

The **Updated** field is the date of the most recent change to the document, as an ISO 8601 extended format calendar date.

### Requires

The conditional **Requires** field lists the numbers of other DLDs whose normative content this one relies on, separated by commas. External sources are listed under References in the document itself.

### Supersedes

The conditional **Supersedes** field is the number of the document this one replaces. Present if and only if this document replaces another.

### Superseded-By

The conditional **Superseded-By** field is the number of the document that replaces this one. Present if and only if the status is Superseded.
