-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
  'ddf8994f-d522-4659-8d02-c1d479057be6',
  'admin',
  '$argon2id$v=19$m=19456,t=2,p=1$ZGmXJZ6LrVlID7VUOv/Bxg$sTuJ0nYjeCpOhZ9eJL+wv/z8921qJxFobA/KH4FXIr0'
);