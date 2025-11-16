-- Add migration script here
ALTER TABLE "threads"
    ADD COLUMN "closed_at" DATETIME NULL;

ALTER TABLE "threads"
    ADD COLUMN "closed_by" TEXT NULL;

ALTER TABLE "threads"
    ADD COLUMN "category_id" TEXT NULL;

ALTER TABLE "threads"
    ADD COLUMN "category_name" TEXT NULL;

ALTER TABLE "threads"
    ADD COLUMN "required_permissions" TEXT NULL;
