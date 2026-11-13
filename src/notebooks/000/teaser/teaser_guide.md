# Teaser Field Guide

## Metadata

notebook: teaser
schema_version: 0.1
status: experimental
purpose: asset_preservation

---

## Block

id: teaser.pattern.escalation
type: mechanism

Statement:

Local
↓
Regional
↓
Mythic
↓
Cosmological

Dependencies:
- teaser.scope
- teaser.compression

Notes:
Primary escalation gradient discovered during teaser development.

Examples:
- town → coast → dreamland → cosmos

Errata:
Escalation survives replacement of references.
Specific lore is not required.

Provenance:
source: teaser_dialogue
category: escalation

## Block

id: teaser.card.too_deep
type: card

Statement:

TOO DEEP

Dependencies:
- teaser.implication
- teaser.compression

Notes:
Implication-first card.
Functions by preserving ambiguity.

Examples:
- TOO OLD
- TOO LATE

Errata:
Card succeeds through omission.
Adding explanation weakens effect.

Provenance:
source: teaser_dialogue
category: compression_card

## Block

id: teaser.concept.guided_incompleteness
type: principle

Statement:

Mystery emerges from guided incompleteness.

Dependencies:
- teaser.audience
- teaser.story

Notes:
Audience supplies missing structure.
Engagement increases when interpretation remains necessary.

Examples:
- unanswered question
- incomplete escalation
- symbolic references

Counterexamples:
- exhaustive exposition
- direct explanation
- lore dumps

Provenance:
source: teaser_abstract
category: ontology

## Block

id: teaser.concept.atmosphere
type: mechanism

Statement:

Atmosphere is emotional topology.

Dependencies:
- teaser.context

Notes:
Atmosphere does not create story.
Atmosphere shapes interpretation.

Examples:
- weight
- expectation
- dread
- wonder

Errata:
Atmosphere amplifies narrative.
Atmosphere is not narrative.

Provenance:
source: teaser_dialogue
category: topology

## Block

id: teaser.concept.narrative_story
type: distinction

Statement:

Narrative = Transmission
Story = Construction

Dependencies:
- edit.narrative
- edit.story

Notes:
Inherited from Editing Ontology.

Examples:
Narrative:
- symbols
- transitions
- implication

Story:
- causality
- identity
- meaning

Errata:
Story is not transmitted.

Counterexample:
Traditional exposition-heavy storytelling.

Provenance:
source: edit_dialogue
category: inherited

## Attachment Candidate

id: teaser.fanservice

Status:
stub

Statement:
Fanservice may function as
a compression mechanism.

Dependencies:
unknown

Notes:
Out of scope.
Preserved solely for subscription continuity.

Future Notebook:
fanservice_ontology

Provenance:
source: teaser_dialogue
category: deferred

## Block

id: teaser.asset.worlds_above_beneath
type: asset

Statement:

WORLDS ABOVE

WORLDS BENEATH

Dependencies:
- teaser.pattern.escalation
- teaser.concept.narrative_story

Notes:
Transition marker between geographic scope
and cosmological scope.

Functions through ontological expansion
rather than location traversal.

Represents a shift from:
place → place

to:

world → world

Audience experiences expansion of possibility space.

Examples:
- WORLDS ABOVE
  WORLDS BENEATH

- BEYOND THE STARS
  BELOW THE SEAS

Errata:
Not a geography card.

Not a lore card.

Not a destination card.

Functions as a scope-transition asset.

Equivalent realizations may vary while preserving role.

Counterexamples:
- DUNWICH
- INNSMOUTH
- DEVIL REEF

Those remain locality markers rather than ontology markers.

Provenance:

Source:
teaser_dialogue

Origin:
original_session

Category:
scope_transition

Status:
realized

Lineage:
teaser.pattern.escalation

## Block

id: teaser.asset.dunwich_sentinel
type: asset

Statement:

FROM THE FIELDS OF DUNWICH

TO THE PILLARS OF SENTINEL HILL

Dependencies:
- teaser.pattern.from_to
- teaser.pattern.escalation

Notes:
Early geographic escalation asset.

Functions through traversal rather than destination.

Establishes movement between localities while preserving ambiguity.

Audience need not understand either reference.

Meaning derives primarily from the implied journey.

Examples:
- FROM THE FIELDS OF DUNWICH
  TO THE PILLARS OF SENTINEL HILL

Variants:
- FROM THE SHORES OF INNSMOUTH
  TO THE DEPTHS OF DEVIL REEF

- FROM THE VAULTS OF ZIN
  TO THE PEAKS OF LENG

Errata:
Dunwich and Sentinel Hill are not required.

The location pair is replaceable.

The relationship survives replacement.

Counterexamples:
DUNWICH

SENTINEL HILL

Presented independently.

Without relational framing the escalation weakens substantially.

Provenance:

Source:
original_teaser_session

Origin:
geographic_escalation_chain

Category:
asset

Status:
realized

Discovery:
teaser.evolution_iii

Lineage:
teaser.pattern.from_to

## Block

id: teaser.asset.innsmouth_reef
type: asset

Statement:

FROM THE SHORES OF INNSMOUTH

TO THE DEPTHS OF DEVIL REEF

Dependencies:
- teaser.pattern.from_to
- teaser.pattern.escalation

Notes:
Geographic escalation asset emphasizing descent.

Unlike Dunwich → Sentinel Hill, which traverses between
surface localities, this chain introduces movement toward
concealed or submerged locality.

Functions through implication of hidden depth rather than
distance alone.

Examples:
- FROM THE SHORES OF INNSMOUTH
  TO THE DEPTHS OF DEVIL REEF

Variants:
- FROM THE VAULTS OF ZIN
  TO THE PEAKS OF LENG

- FROM THE FIELDS OF DUNWICH
  TO THE PILLARS OF SENTINEL HILL

Errata:
The references themselves are replaceable.

The surviving structure is:

surface
↓
concealed

known
↓
hidden

Counterexamples:
INNSMOUTH

DEVIL REEF

Presented independently.

Without relational framing, the descent pattern weakens.

Provenance:

Source:
original_teaser_session

Origin:
geographic_escalation_chain

Category:
asset

Status:
realized

Discovery:
teaser.evolution_iii

Lineage:
teaser.pattern.from_to

## Block

id: teaser.asset.zin_leng
type: asset

Statement:

FROM THE VAULTS OF ZIN

TO THE PEAKS OF LENG

Dependencies:
- teaser.pattern.from_to
- teaser.pattern.escalation

Notes:
Mythic escalation asset emphasizing ascent and elevation.

Unlike:

DUNWICH → SENTINEL HILL
which traverses between localities,

and:

INNSMOUTH → DEVIL REEF
which descends toward concealment,

this chain implies movement toward remoteness,
elevation, and mythic significance.

Functions through increasing symbolic altitude.

Examples:
- FROM THE VAULTS OF ZIN
  TO THE PEAKS OF LENG

Variants:
- FROM THE FIELDS OF DUNWICH
  TO THE PILLARS OF SENTINEL HILL

- FROM THE SHORES OF INNSMOUTH
  TO THE DEPTHS OF DEVIL REEF

Errata:
The references themselves are replaceable.

The preserved structure is:

buried
↓
elevated

hidden
↓
revealed

local
↓
mythic

Counterexamples:
ZIN

LENG

Presented independently.

Without relational framing, ascent weakens.

Provenance:

Source:
original_teaser_session

Origin:
geographic_escalation_chain

Category:
asset

Status:
realized

Discovery:
teaser.evolution_iii

Lineage:
teaser.pattern.from_to

## Block

id: teaser.family.too_deep_old_late
type: family

Statement:

TOO DEEP

TOO OLD

TOO LATE

Dependencies:
- teaser.concept.guided_incompleteness
- teaser.pattern.escalation

Notes:
Compression family discovered through implication-first teaser design.

The three cards function as a single structure rather than three independent assets.

Each member follows:

TOO + QUALIFIER

The audience is forced to supply:

- subject
- cause
- consequences
- context

This preserves ambiguity while transmitting narrative pressure.

The family demonstrates how meaning density can increase while
information density decreases.

Examples:

TOO DEEP

TOO OLD

TOO LATE

Potential Variants:

TOO FAR

TOO SOON

TOO LARGE

TOO CLOSE

Associations:

TOO DEEP
→ unknown depth
→ abyss
→ concealment
→ inaccessibility

TOO OLD
→ antiquity
→ forgotten history
→ persistence
→ inheritance

TOO LATE
→ consequence
→ inevitability
→ closure
→ failure of intervention

Errata:

The individual cards are not the durable object.

The durable object is:

TOO + unresolved condition

The family functions through omission rather than description.

Adding explicit information typically weakens effect.

Counterexamples:

DEPTH 12000 FEET

ANCIENT CIVILIZATION

IMPACT IN 30 SECONDS

These describe.
The family implies.

What Survived:

Implication

Compression

Audience Construction

Guided Incompleteness

Mystery Through Omission

What Was Rejected:

Information-first delivery

Direct exposition

Explicit narrative framing

Provenance:

Source:
original_teaser_session

Origin:
implication_first_cards

Category:
compression_family

Status:
realized

Discovery:
teaser.evolution_i

Lineage:
teaser.card.too_deep

## Block

id: teaser.family.signal_response_warning
type: family

Statement:

A SIGNAL

A RESPONSE

A WARNING

Dependencies:
- teaser.concept.narrative_story
- teaser.pattern.escalation

Notes:
Narrative compression family expressing causal escalation.

Unlike geographic escalation chains, which expand scope through
movement, this family expands scope through causality.

Each element increases implied structure while adding minimal
explicit information.

Progression:

A SIGNAL
↓
A RESPONSE
↓
A WARNING

The audience naturally infers:

origin
↓
agency
↓
consequence

without any explicit exposition.

Functions through progressive implication rather than
direct explanation.

Examples:

A SIGNAL

A RESPONSE

A WARNING

Potential Variants:

A QUESTION
A REPLY
A CONSEQUENCE

A KNOCK
AN ANSWER
AN ARRIVAL

THEY WATCHED
THEY WAITED
THEY ARRIVED

Errata:

The words themselves are replaceable.

The preserved structure is:

event
↓
reaction
↓
significance

or:

observation
↓
agency
↓
implication

Counterexamples:

AN ANCIENT TRANSMISSION WAS DETECTED

THE SIGNAL ORIGINATED FROM AN UNKNOWN SOURCE

THE SOURCE PRESENTS A THREAT

Explanation weakens participation.

The family succeeds by forcing audience inference.

What Survived:

Causal Escalation

Narrative Compression

Audience Construction

Guided Incompleteness

Minimal Signal
Maximum Inference

What Was Rejected:

Exposition-heavy progression

Direct causal explanation

Lore-first delivery

Provenance:

Source:
original_teaser_session

Origin:
narrative_compression_exploration

Category:
compression_family

Status:
realized

Discovery:
teaser.evolution_i

Lineage:
teaser.concept.guided_incompleteness

## Block

id: teaser.family.watched_waited_arrived_left
type: family

Statement:

THEY WATCHED

THEY WAITED

THEY ARRIVED

THEY NEVER LEFT

Dependencies:
- teaser.concept.guided_incompleteness
- teaser.concept.narrative_story
- teaser.family.signal_response_warning

Notes:
Narrative compression family expressing persistent agency.

Unlike:

A SIGNAL
A RESPONSE
A WARNING

which escalates through causal implication,

this family escalates through implied presence.

Each successive statement contributes minimal information while increasing:

- agency
- duration
- significance
- inevitability

The final line recontextualizes all preceding lines.

Progression:

observation
↓
duration
↓
manifestation
↓
revelation

The audience is encouraged to construct:

who
↓
why
↓
how long
↓
what changed

without direct exposition.

Examples:

THEY WATCHED

THEY WAITED

THEY ARRIVED

THEY NEVER LEFT

Potential Variants:

IT WAITED

IT REMEMBERED

IT RETURNED

IT WAS ALWAYS HERE

---

WE SAW

WE FOLLOWED

WE FOUND

WE WERE FOUND

---

THEY CAME

THEY STAYED

THEY CHANGED

THEY REMAIN

Errata:

The strength of the family resides primarily in the final line.

The final realization retroactively alters the interpretation of previous statements.

The durable structure is:

presence
↓
continuity
↓
manifestation
↓
recontextualization

or:

observation
↓
expectation
↓
event
↓
revelation

The wording may change.

The behavioral role survives.

Counterexamples:

AN ENTITY OBSERVED HUMANITY

IT REMAINED DORMANT FOR CENTURIES

IT EMERGED

IT STILL EXISTS

These explain rather than imply.

Explanation reduces participation.

The family succeeds because:
- agency remains unspecified
- causality remains incomplete
- audience construction remains necessary

What Survived:

Agency Escalation

Narrative Compression

Retrospective Reinterpretation

Audience Construction

Compression Through Omission

What Was Rejected:

Explicit chronology

Direct exposition

Lore-first delivery

Complete causal explanation

Provenance:

Source:
original_teaser_session

Origin:
narrative_compression_exploration

Category:
agency_family

Status:
realized

Discovery:
teaser.evolution_i

Lineage:
teaser.family.signal_response_warning

## Block

id: teaser.family.not_lost_waiting
type: family

Statement:

NOT LOST

WAITING

Dependencies:
- teaser.concept.guided_incompleteness
- teaser.concept.narrative_story

Notes:
Narrative compression family expressing preserved relevance.

Unlike:

TOO DEEP
TOO OLD
TOO LATE

which compresses condition,

A SIGNAL
A RESPONSE
A WARNING

which compresses causality,

and

THEY WATCHED
THEY WAITED
THEY ARRIVED
THEY NEVER LEFT

which compresses agency across time,

this family compresses reclassification and renewed attention.

The audience is encouraged to revise an earlier assumption.

Progression:

absence
↓
presence

loss
↓
availability

failure
↓
continuation

dismissal
↓
significance

Examples:

NOT LOST

WAITING

Potential Variants:

NOT GONE

WATCHING

---

NOT DEAD

SLEEPING

---

NOT FORGOTTEN

REMEMBERING

---

NOT BROKEN

UNFINISHED

Errata:

The family operates through reinterpretation.

The second statement does not add information.

The second statement redirects attention.

The durable structure is:

incorrect assumption
↓
corrected significance

or:

dismissal
↓
renewed relevance

Counterexamples:

THEY WERE NEVER LOST

THEY CHOSE TO WAIT

THEY WILL RETURN LATER

Direct explanation weakens participation.

The family succeeds because the audience performs
the reinterpretation themselves.

What Survived:

Attention Preservation

Recontextualization

Narrative Pressure

Audience Revision

Compression Through Reclassification

Guided Reinterpretation

What Was Rejected:

Explicit clarification

Direct exposition

Premature resolution

Complete causal framing

Provenance:

Source:
original_teaser_session

Origin:
compression_family_exploration

Category:
attention_family

Status:
realized

Discovery:
teaser.evolution_v

Lineage:
teaser.concept.guided_incompleteness

## Block

id: teaser.family.almost_threshold
type: family

Statement:

ALMOST FOUND

ALMOST SEEN

ALMOST OPEN

Dependencies:
- teaser.concept.guided_incompleteness
- teaser.concept.narrative_story

Notes:
Narrative compression family expressing proximity to transition.

Unlike:

TOO DEEP
TOO OLD
TOO LATE

which compresses condition,

and:

NOT LOST
WAITING

which compresses reclassification,

this family compresses imminence.

The audience is positioned at the edge of change.

Progression is implicit rather than explicit.

The family functions by creating expectation pressure.

Examples:

ALMOST FOUND

ALMOST SEEN

ALMOST OPEN

Potential Variants:

NEARLY REMEMBERED

NEARLY RETURNED

NEARLY UNDERSTOOD

---

ONE STEP MORE

JUST BEYOND

NOT YET

---

SO CLOSE

STILL HIDDEN

Errata:

The family is not about revelation.

It is about delayed revelation.

The durable structure is:

approach
↓
threshold

or:

unresolved
↓
imminent

Counterexamples:

THE DOOR OPENED

THEY FOUND IT

THE SECRET WAS REVEALED

Resolution collapses the pressure.

The family succeeds because the transition remains incomplete.

What Survived:

Threshold Pressure

Expectation

Deferred Resolution

Audience Anticipation

Compression Through Imminence

What Was Rejected:

Complete revelation

Immediate resolution

Explanation-first framing

Provenance:

Source:
teaser_exploration

Origin:
compression_family_exploration

Category:
threshold_family

Status:
candidate

Discovery:
teaser.evolution_v

Lineage:
teaser.concept.guided_incompleteness

## Block

id: teaser.family.it_was
type: family

Statement:

IT WAS HERE

IT WAS US

IT WAS OPEN

Dependencies:
- teaser.family.not_lost_waiting
- teaser.concept.narrative_story

Notes:
Narrative compression family expressing revelation.

Unlike:

NOT LOST
WAITING

which preserves attention,

or:

THEY WATCHED
THEY WAITED
THEY ARRIVED
THEY NEVER LEFT

which preserves agency,

this family resolves ambiguity through recognition.

The structure delivers a missing relationship rather than a missing event.

Progression:

uncertainty
↓
recognition

absence
↓
presence

separation
↓
identity

The audience experiences:

"I understand."

rather than:

"What happens next?"

Examples:

IT WAS HERE

IT WAS US

IT WAS OPEN

Potential Variants:

WE WERE WRONG

IT WAS ALWAYS THERE

IT WAS NEVER LOST

---

THE DOOR WAS OPEN

THE SIGNAL WAS OURS

THEY WERE US

Errata:

The family operates through reinterpretation and closure.

Unlike threshold families, which approach revelation,

this family performs revelation.

The durable structure is:

mystery
↓
recognition

or:

question
↓
answer

Counterexamples:

MAYBE

ALMOST FOUND

JUST BEYOND

These preserve tension rather than resolving it.

What Survived:

Recognition

Revelation

Closure

Retrospective Coherence

Narrative Resolution

What Was Rejected:

Extended ambiguity

Perpetual escalation

Infinite deferral

Provenance:

Source:
teaser_exploration

Origin:
resolution_taxonomy

Category:
revelation_family

Status:
candidate

Discovery:
taxonomy_phase

Lineage:
teaser.family.not_lost_waiting

## Block

id: teaser.family.at_last
type: family

Statement:

AT LAST

IT IS DONE

THE END

Dependencies:
- teaser.family.it_was
- teaser.concept.narrative_story

Notes:
Narrative compression family expressing closure.

Unlike:

IT WAS HERE
IT WAS US
IT WAS OPEN

which resolves ambiguity through revelation,

this family resolves narrative pressure itself.

The audience is no longer asked:

"What happened?"

or

"What does it mean?"

Instead the structure communicates:

"There is no further obligation to search."

Progression:

uncertainty
↓
understanding
↓
completion

or:

tension
↓
release

Examples:

AT LAST

IT IS DONE

THE END

Potential Variants:

FINALLY

NO MORE QUESTIONS

THE SEARCH IS OVER

---

FOUND

UNDERSTOOD

FINISHED

---

HOME

RETURNED

REST

Errata:

The family does not reveal.

The family terminates revelation.

The durable structure is:

resolution
↓
acceptance

or:

closure
↓
stillness

Counterexamples:

IT WAS US

IT WAS HERE

THEY NEVER LEFT

These resolve interpretation but continue generating narrative pressure.

The closure family intentionally collapses future inquiry.

What Survived:

Closure

Completion

Acceptance

Release

Narrative Termination

What Was Rejected:

Infinite escalation

Perpetual mystery

Endless reinterpretation

Recursive revelation

Provenance:

Source:
taxonomy_phase

Origin:
closure_taxonomy

Category:
closure_family

Status:
candidate

Discovery:
taxonomy_expansion

Lineage:
teaser.family.it_was

## Block

id: teaser.family.become_other
type: family

Statement:

NO LONGER

BECOMING

SOMETHING ELSE

Dependencies:
- teaser.family.after_the_reveal
- teaser.concept.narrative_story

Notes:
Narrative compression family expressing transformation.

Unlike:

IT WAS HERE
IT WAS US
IT WAS OPEN

which resolves uncertainty,

or:

NOW WE KNOW
NOTHING CHANGED
EVERYTHING CHANGED

which explores aftermath,

this family focuses on altered identity.

The revelation has already occurred.

The consequences have begun.

Now the question becomes:

What has changed?

Examples:

NO LONGER

BECOMING

SOMETHING ELSE

Potential Variants:

NOT WHAT WE WERE

CHANGED

UNRECOGNIZABLE

---

A DIFFERENT SHAPE

A DIFFERENT NAME

A DIFFERENT WORLD

---

WE RETURNED

CHANGED

FOREVER

Errata:

The family does not describe events.

The family describes state transition.

The durable structure is:

identity
↓
mutation

or:

recognition
↓
transformation

The audience is encouraged to infer:

what changed

why it changed

who changed

without requiring direct exposition.

Counterexamples:

IT MOVED

IT ARRIVED

IT OPENED

These describe events.

Transformation concerns altered identity rather than occurrence.

What Survived:

Metamorphosis

Identity Shift

State Transition

Aftermath Persistence

Transformation Pressure

What Was Rejected:

Static identity

Return to previous state

Pure event framing

Transformation without consequence

Provenance:

Source:
taxonomy_phase

Origin:
transformation_taxonomy

Category:
transformation_family

Status:
candidate

Discovery:
taxonomy_expansion

Lineage:
teaser.family.after_the_reveal

## Block

id: teaser.family.still_here
type: family

Statement:

STILL HERE

UNCHANGED

AS BEFORE

Dependencies:
- teaser.family.become_other
- teaser.concept.narrative_story

Notes:
Narrative compression family expressing identity persistence.

Unlike:

NO LONGER

BECOMING

SOMETHING ELSE

which describes transformation,

this family preserves continuity across transformation.

The audience is invited to infer:

what changed

and

what survived.

Progression:

transformation
↓
continuity

or:

change
↓
identity

Examples:

STILL HERE

UNCHANGED

AS BEFORE

Potential Variants:

STILL WAITING

STILL WATCHING

STILL THERE

---

DIFFERENT

YET THE SAME

---

NOT WHAT IT WAS

NOT WHAT IT SEEMED

STILL ITSELF

Errata:

The family does not deny transformation.

It asserts persistence through transformation.

The durable structure is:

mutation
↓
continuity

or:

difference
↓
identity

Counterexamples:

SOMETHING ELSE

UNRECOGNIZABLE

NO LONGER

These emphasize discontinuity rather than persistence.

What Survived:

Identity

Continuity

Recognition

Persistence Through Change

Structural Invariance

What Was Rejected:

Total replacement

Absolute discontinuity

Transformation without inheritance

Provenance:

Source:
taxonomy_phase

Origin:
identity_persistence_taxonomy

Category:
identity_family

Status:
candidate

Discovery:
taxonomy_expansion

Lineage:
teaser.family.become_other
