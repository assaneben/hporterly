-- Migration: hierarchical spatial model for referential services
-- Site > Building > Level > Zone/Service > Sub-zone

ALTER TABLE referential_services
    ADD COLUMN site_id VARCHAR(100),
    ADD COLUMN site_name VARCHAR(255),
    ADD COLUMN building_id VARCHAR(100),
    ADD COLUMN building_name VARCHAR(255),
    ADD COLUMN level_id VARCHAR(100),
    ADD COLUMN level_name VARCHAR(255),
    ADD COLUMN zone_id VARCHAR(100),
    ADD COLUMN zone_name VARCHAR(255),
    ADD COLUMN subzone_id VARCHAR(100),
    ADD COLUMN subzone_name VARCHAR(255);

WITH normalized AS (
    SELECT
        id,
        COALESCE(NULLIF(site_name, ''), 'Site principal') AS n_site_name,
        COALESCE(NULLIF(building_name, ''), NULLIF(building, ''), 'Batiment principal') AS n_building_name,
        COALESCE(NULLIF(level_name, ''), NULLIF(floor, ''), 'Niveau inconnu') AS n_level_name,
        COALESCE(NULLIF(zone_name, ''), NULLIF(name, ''), 'Zone non renseignee') AS n_zone_name,
        NULLIF(COALESCE(subzone_name, ''), '') AS n_subzone_name
    FROM referential_services
)
UPDATE referential_services rs
SET
    site_name = n.n_site_name,
    site_id = COALESCE(NULLIF(rs.site_id, ''), CONCAT('SITE-', UPPER(REGEXP_REPLACE(n.n_site_name, '[^A-Za-z0-9]+', '-', 'g')))),
    building_name = n.n_building_name,
    building_id = COALESCE(NULLIF(rs.building_id, ''), CONCAT('BAT-', UPPER(REGEXP_REPLACE(n.n_building_name, '[^A-Za-z0-9]+', '-', 'g')))),
    level_name = n.n_level_name,
    level_id = COALESCE(NULLIF(rs.level_id, ''), CONCAT('NIV-', UPPER(REGEXP_REPLACE(n.n_level_name, '[^A-Za-z0-9]+', '-', 'g')))),
    zone_name = n.n_zone_name,
    zone_id = COALESCE(NULLIF(rs.zone_id, ''), CONCAT('ZONE-', UPPER(REGEXP_REPLACE(n.n_zone_name, '[^A-Za-z0-9]+', '-', 'g')))),
    subzone_name = n.n_subzone_name,
    subzone_id = CASE
        WHEN n.n_subzone_name IS NULL THEN NULL
        ELSE COALESCE(
            NULLIF(rs.subzone_id, ''),
            CONCAT('SUB-', UPPER(REGEXP_REPLACE(n.n_subzone_name, '[^A-Za-z0-9]+', '-', 'g')))
        )
    END
FROM normalized n
WHERE rs.id = n.id;

ALTER TABLE referential_services
    ALTER COLUMN site_id SET NOT NULL,
    ALTER COLUMN site_name SET NOT NULL,
    ALTER COLUMN building_id SET NOT NULL,
    ALTER COLUMN building_name SET NOT NULL,
    ALTER COLUMN level_id SET NOT NULL,
    ALTER COLUMN level_name SET NOT NULL,
    ALTER COLUMN zone_id SET NOT NULL,
    ALTER COLUMN zone_name SET NOT NULL;

CREATE INDEX IF NOT EXISTS idx_ref_services_hierarchy_lookup
    ON referential_services (is_active, site_id, building_id, level_id, zone_id);
