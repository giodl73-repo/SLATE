# Frozen NCES public aggregate source

Publisher: U.S. Department of Education, National Center for Education Statistics.
CCD Table 2, school years 2023–24 and 2024–25, Final Version 1a. URLs, capture
time, byte lengths and SHA-256 hashes are in source-custody.json. Exact HTML
originals are retained; no images or external resources are copied/executed by
the browser task. Normalized JSON is deterministically derived by
`python tools/nces_source.py --write`; build and verification re-extract it.

The table combines operating school/LEA directory counts and SEA membership/staff
aggregates. Student membership and teacher FTE include Pre-K through grade 13;
adult-education students are excluded. FTE displays retain the table's integer
precision; they are not teacher headcounts or exact staffing microdata.
Pupil/teacher ratios are published source values, not class sizes or staffed
seats. No program, grade, support, eligibility, access, quality, outcome, equity,
affordability, funding or governance conclusion is established.

US totals cover 50 states plus DC. Seven other jurisdiction/agency rows stay
separate. Em-dash unavailable and double-dagger withheld markers are preserved
with source notes. In 2024–25, Tennessee's absent teacher data holds national
teacher totals/ratios; Guam membership is suppressed for source quality. No
partial national teacher sum or inferred ratio replaces a missing observation.

Attribution: NCES CCD Table 2, Final Version 1a (2023–24 and 2024–25). Official
federal statistical table contents are reproduced with attribution; SLATE's
original code/content licenses do not relicense the source data. Source pages
retain their own notices. No endorsement is implied. NCES interpretation context:
https://ies.ed.gov/learn/blog/changes-pupilteacher-ratios-2020-impacts-covid-19-pandemic
and the original table notes. This is a frozen repository capture, not a live
feed or a claim to the latest revision.
