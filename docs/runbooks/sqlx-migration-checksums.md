# sqlx migration checksum reconciliation (20270103 to 20270106)

## What happened

The one-line header comment of four migrations was changed in merge commit `92c6bb2` (ADR
renumbering) and restored to the original text in `19c8efc`. Commit `92c6bb2` was pushed to PR #12
and stayed there for a while. sqlx stores the SHA-384 of each migration file in
`_sqlx_migrations.checksum` and refuses to start when an applied migration no longer matches:

```
migration 20270103000001 was previously applied but has been modified
```

* A database that applied these migrations while the tree was at `92c6bb2` recorded the
  post-merge checksums. It now fails with the error above until reconciled (this runbook).
* A database that applied the original files (before `92c6bb2`, or from `19c8efc` onwards) is
  fine and needs no action.

The headers cite the ADR number each file had when it was first committed (0025, 0024, 0026,
0027). They are frozen on purpose: never edit a file under `migrations/`, even a comment.

## 1. Which case are you in?

Hashes of the two file versions (generated with `sha384sum`, not typed by hand):

| Version | File | SHA-384 at 92c6bb2 (post-merge) | SHA-384 now (original) |
|---|---|---|---|
| 20270103000001 | `20270103000001_packaging_shipping.sql` | `d698b54d500a6fed81fd2329b2001242b412f46a0e42b0445c612d75b09ff8df9711f52b0cf9cbb66a895560c3a51a98` | `b9901c573ba7bcd685d13739381d1b868bb50dacde24b924d7ed6725e0fcd9b2cc3663934021c4a81e8734b88c3bad6b` |
| 20270104000001 | `20270104000001_power_grids.sql` | `c344026d098033bbe89dc43cbeaf0f3bb056bd3cace3ed07789da086958589e981c19c3beb0a3e1cdc6f0dcc4cace2f9` | `1720993ed68a5a54fe6fa3c94c5a5c254bd8edb1db828b6ad1bbd8a1a27347efb218cb5bfe1368a978c6bbc91003a6bc` |
| 20270105000001 | `20270105000001_product_relations.sql` | `3436e3c3828881712b44ba11f8c9a3254de4d3a6b4dc7c16733a1ac44ca8445d761357000dd35bd59ccbc10225742567` | `98cc5b8fb56cf0a3ec0750c4d3e1e5c1ae48ce467ef2d422fb01a2e3029525b9674378c7f4a546a1f9e1a0ee7e13ab9f` |
| 20270106000001 | `20270106000001_modules_users_admin.sql` | `294a819747733a67547699c931a7d4041f49a5ae2d8b10423cfaf8c263e06ef271d886c308da9e9179faccf3a863efd0` | `0954f7f579ce4bd31b7535e63e718004cfef221e65ec1f050043c21db6ab078f82ddb46f122e90644ea4a4ba49c8abf8` |

Compute them yourself from a checkout of the current branch:

```bash
for f in migrations/2027010[3-6]*.sql; do
  echo "$f"
  echo "  92c6bb2: $(git show 92c6bb2:$f | sha384sum | cut -d' ' -f1)"
  echo "  current: $(sha384sum $f | cut -d' ' -f1)"
done
```

What the database recorded:

```bash
psql "$DATABASE_URL" -Atc "select version, encode(checksum,'hex') from _sqlx_migrations
  where version in (20270103000001,20270104000001,20270105000001,20270106000001) order by 1"
```

Compare per version:

* stored equals the **current** hash: fine, nothing to do for that version.
* stored equals the **92c6bb2** hash: affected, reconcile it (step 2).
* stored equals neither: stop. The file was modified in some other way; do not run the UPDATE.
* no row: the migration was not applied yet; sqlx applies it normally on start.

## 2. Reconcile (affected versions only)

1. Stop the API (and worker) so nothing runs the migrator meanwhile.
2. Back up first:

   ```bash
   pg_dump -Fc "$DATABASE_URL" -f tradocraft-before-checksum-fix.dump
   ```

3. Run the statements below in one transaction, in this order (20270103, 20270104, 20270105,
   20270106). Each `UPDATE` only matches when the stored checksum equals the post-merge hash, so a
   version that is already correct or unapplied is left untouched (0 rows). Check that the
   reported counts match what step 1 found.

   ```sql
   BEGIN;

   UPDATE _sqlx_migrations
      SET checksum = decode('b9901c573ba7bcd685d13739381d1b868bb50dacde24b924d7ed6725e0fcd9b2cc3663934021c4a81e8734b88c3bad6b', 'hex')
    WHERE version = 20270103000001
      AND checksum = decode('d698b54d500a6fed81fd2329b2001242b412f46a0e42b0445c612d75b09ff8df9711f52b0cf9cbb66a895560c3a51a98', 'hex');

   UPDATE _sqlx_migrations
      SET checksum = decode('1720993ed68a5a54fe6fa3c94c5a5c254bd8edb1db828b6ad1bbd8a1a27347efb218cb5bfe1368a978c6bbc91003a6bc', 'hex')
    WHERE version = 20270104000001
      AND checksum = decode('c344026d098033bbe89dc43cbeaf0f3bb056bd3cace3ed07789da086958589e981c19c3beb0a3e1cdc6f0dcc4cace2f9', 'hex');

   UPDATE _sqlx_migrations
      SET checksum = decode('98cc5b8fb56cf0a3ec0750c4d3e1e5c1ae48ce467ef2d422fb01a2e3029525b9674378c7f4a546a1f9e1a0ee7e13ab9f', 'hex')
    WHERE version = 20270105000001
      AND checksum = decode('3436e3c3828881712b44ba11f8c9a3254de4d3a6b4dc7c16733a1ac44ca8445d761357000dd35bd59ccbc10225742567', 'hex');

   UPDATE _sqlx_migrations
      SET checksum = decode('0954f7f579ce4bd31b7535e63e718004cfef221e65ec1f050043c21db6ab078f82ddb46f122e90644ea4a4ba49c8abf8', 'hex')
    WHERE version = 20270106000001
      AND checksum = decode('294a819747733a67547699c931a7d4041f49a5ae2d8b10423cfaf8c263e06ef271d886c308da9e9179faccf3a863efd0', 'hex');

   COMMIT;
   ```

   Run it with `psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f fix.sql`. Expected output is
   `UPDATE 1` for each affected version and `UPDATE 0` for each one that is not.
4. Re-run the query from step 1: every stored value must now equal the current hash.
5. Start the API again. The migrator validates every applied checksum and then applies anything
   still pending.

Rollback if anything looks wrong: `pg_restore --clean --if-exists -d "$DATABASE_URL" tradocraft-before-checksum-fix.dump`.

## What was verified

On a scratch PostgreSQL database (created over the unix socket, dropped afterwards), using the
sqlx 0.8 `Migrator` through a throwaway program kept outside the repository:

1. Applied all migrations as they are at `92c6bb2`: migrator succeeded; `_sqlx_migrations` held the
   post-merge hashes in the table above for the four versions (24 rows in total).
2. Ran the migrator with the current `migrations/` directory: failed with
   `migration 20270103000001 was previously applied but has been modified`.
3. Ran the transaction from step 2 (the statements are the ones in this file, extracted
   verbatim): `UPDATE 1` four times, `COMMIT`.
4. Ran the migrator again: succeeded; still 24 rows in `_sqlx_migrations`, the four checksums
   equal to the current hashes, `success` true, other rows unchanged.
5. Unaffected case: applied the current files to a fresh scratch database, the migrator ran with
   no error and a second run was a no-op. The guarded UPDATEs there matched `UPDATE 0` four times
   and changed nothing.
