ALTER TABLE contraption_tag DROP CONSTRAINT contraptionstags_tags_id_fkey,
ADD CONSTRAINT contraptionstags_tags_id_fkey FOREIGN KEY (tag_id) REFERENCES tag(id)