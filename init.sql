CREATE TABLE IF NOT EXISTS "todos" (
    "id" uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    "message" TEXT NOT NULL,
    "done" BOOL NOT NULL DEFAULT FALSE
);
