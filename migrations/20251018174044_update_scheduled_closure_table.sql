-- Add migration script here
ALTER TABLE "scheduled_closures"
    ADD COLUMN "closed_at" DATETIME NULL;

ALTER TABLE "scheduled_closures"
    ADD COLUMN "closed_by" TEXT NULL;

ALTER TABLE "scheduled_closures"
    ADD COLUMN "category_id" TEXT NULL;

ALTER TABLE "scheduled_closures"
    ADD COLUMN "category_name" TEXT NULL;

ALTER TABLE "scheduled_closures"
    ADD COLUMN "required_permissions" TEXT NULL;