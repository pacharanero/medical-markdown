---
type: Decision
title: MIT licence, diverging from the AGPL house default
description: The repository is MIT-licensed as a deliberate divergence from the house-style AGPL-3.0-or-later default, to keep the format and parser embeddable.
tags: [decision, licence]
timestamp: '2026-07-28T01:15:00Z'
---

# Decision

This repository is licensed under **MIT**, a deliberate divergence from
the house-style default of AGPL-3.0-or-later.

# Rationale

Medical Markdown is intended as a permissive, embeddable format and
reference parser. MIT lowers the barrier to adoption in both open source
and proprietary EPR systems. Concretely: GitEHR is itself AGPL-3.0 and can
depend on this crate without licence friction — recorded as a
non-requirement to preserve in the
[GitEHR embedding requirements](/spec/gitehr-embedding.md).

# Citations

[1] [README — License](https://github.com/folkengine/medical-markdown/blob/master/README.md)
[2] [LICENSE](https://github.com/folkengine/medical-markdown/blob/master/LICENSE)
