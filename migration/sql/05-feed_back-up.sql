CREATE TABLE "user_feed_back" (
 "id" serial NOT NULL PRIMARY KEY,
 "user_id" VARCHAR(50) NOT NULL,
 "app_name" VARCHAR(50) NOT NULL,
 "module" VARCHAR(50) NOT NULL,
 "content" VARCHAR(500) NOT NULL,
 "contact" VARCHAR(50) NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE INDEX idx_user_fb_user ON "user_feed_back" ("user_id");


COMMENT ON TABLE "user_feed_back" IS '';
COMMENT ON COLUMN "user_feed_back"."id" IS '';
COMMENT ON COLUMN "user_feed_back"."user_id" IS '用户ID';
COMMENT ON COLUMN "user_feed_back"."module" IS '用名呢称';
COMMENT ON COLUMN "user_feed_back"."content" IS '';
COMMENT ON COLUMN "user_feed_back"."contact" IS '用名呢称';
COMMENT ON COLUMN "user_feed_back"."created" IS '创建时间';
COMMENT ON COLUMN "user_feed_back"."updated" IS '更新时间';
