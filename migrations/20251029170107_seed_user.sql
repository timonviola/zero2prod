-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    'ddaa3106-aea4-4bd7-b420-c897195528f0',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$mYRlx3OYPZzv3QQCCSCzhw$CpuNembJw1YdsSCeYfO1x/huL3hP7A/vJx0FxOP4CGE'
)
