# Skill: Platform / Add Migration And Seed

Use this workflow when schema or local development data must change.

## Trigger

- a new entity or field requires schema support
- transform metadata storage changes
- creator/editor flows need new seed data for local development

## Checklist

1. Write the migration with a clear forward path.
2. Write or confirm the rollback path.
3. Update seed data only if schema or defaults require it.
4. Verify local developer setup still works with the changed schema.
5. Check whether creator, editor, or API payload assumptions changed.

## Repo touchpoints

- `database/audio_db/migrations`
- `database/seeds`
- backend data providers and DTOs

## Done when

- migration path is valid
- seed path remains usable
- affected payloads still round-trip correctly

## Minimum verification

- `make migrate-info`
- apply the migration in a dev environment
- run the relevant seed flow
- verify changed entities can still be read and written end to end
