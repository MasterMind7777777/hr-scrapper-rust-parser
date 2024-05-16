-- Add the html_hash column to the pages table
ALTER TABLE pages
ADD COLUMN html_hash VARCHAR NOT NULL;
