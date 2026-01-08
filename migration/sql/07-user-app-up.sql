CREATE TABLE "user_app" (
 "id" serial NOT NULL PRIMARY KEY,
 "user_id" VARCHAR(50) NOT NULL,
 "app_id" VARCHAR(50) NOT NULL,
 "remark" VARCHAR(500),
 "status" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);


COMMENT ON TABLE "user_app" IS '用户开通应用';
COMMENT ON COLUMN "user_app"."id" IS '';
COMMENT ON COLUMN "user_app"."user_id" IS '用户ID';
COMMENT ON COLUMN "user_app"."app_id" IS '';
COMMENT ON COLUMN "user_app"."remark" IS '';
COMMENT ON COLUMN "user_app"."status" IS '0 禁用  1启用';
COMMENT ON COLUMN "user_app"."created" IS '创建时间';
COMMENT ON COLUMN "user_app"."updated" IS '更新时间';
