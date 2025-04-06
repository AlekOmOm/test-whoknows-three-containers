CREATE TABLE search_results (
    id INT IDENTITY(1,1) PRIMARY KEY,
    title NVARCHAR(MAX) NOT NULL,
    description NVARCHAR(MAX) NOT NULL,
    url NVARCHAR(MAX) NOT NULL,
    keywords NVARCHAR(MAX)
);

-- Add some sample data
INSERT INTO search_results (title, description, url, keywords)
VALUES 
('Example Search Result', 'This is a sample result description', 'https://example.com', '["example", "result"]'),
('Another Result', 'This is another sample result description', 'https://another.com', '["another", "result"]'),
('Third Result', 'This is the third sample result description', 'https://third.com', '["third", "result"]')