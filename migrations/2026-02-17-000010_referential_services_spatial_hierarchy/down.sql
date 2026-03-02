DROP INDEX IF EXISTS idx_ref_services_hierarchy_lookup;

ALTER TABLE referential_services
    DROP COLUMN IF EXISTS subzone_name,
    DROP COLUMN IF EXISTS subzone_id,
    DROP COLUMN IF EXISTS zone_name,
    DROP COLUMN IF EXISTS zone_id,
    DROP COLUMN IF EXISTS level_name,
    DROP COLUMN IF EXISTS level_id,
    DROP COLUMN IF EXISTS building_name,
    DROP COLUMN IF EXISTS building_id,
    DROP COLUMN IF EXISTS site_name,
    DROP COLUMN IF EXISTS site_id;
