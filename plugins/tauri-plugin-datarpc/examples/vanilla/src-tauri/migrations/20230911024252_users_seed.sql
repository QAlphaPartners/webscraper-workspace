-- Add migration script here
-- User demo1
-- INSERT INTO "user" (username,pwd,pwd_salt,token_salt) VALUES ('demo1','welcome','demo1_pwd_salt','demo1_token_salt')  RETURNING id;
INSERT INTO "user" (username,pwd,pwd_salt,token_salt) VALUES ('demo1','#01#EcOKbwYSyfOFHWlN2jYjccRdXt9Zb8ixzTlxJtQcG-YCZO1CmxEaS9TXWI8WiR2qHEvkS0iRoCJNmCclH86NHQ','demo1_pwd_salt','demo1_token_salt')  RETURNING id;