ALTER TABLE contraption_item RENAME COLUMN contraption_id TO contraptions_id;
ALTER TABLE contraption_item RENAME COLUMN item_id TO items_id;
ALTER TABLE contraption_tag RENAME COLUMN contraption_id TO contraptions_id;
ALTER TABLE contraption_tag RENAME COLUMN tag_id TO tags_id;