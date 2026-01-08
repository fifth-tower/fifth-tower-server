CREATE TABLE "group" (
 "group_id" VARCHAR(50) NOT NULL PRIMARY KEY,
 "group_name" VARCHAR(50) NOT NULL,
 "remark" VARCHAR(500),
 "status" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);


CREATE TABLE "role" (
 "role_id" VARCHAR(50) NOT NULL PRIMARY KEY,
 "role_name" VARCHAR(50) NOT NULL,
 "remark" VARCHAR(500),
 "status" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);


CREATE TABLE "user_group" (
 "user_id" VARCHAR(50) NOT NULL PRIMARY KEY,
 "group_id" VARCHAR(50) NOT NULL,
 "status" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE INDEX idx_user_group_g ON "user_group" ("group_id");


CREATE TABLE "user_role" (
 "id" serial NOT NULL PRIMARY KEY,
 "user_id" VARCHAR(50) NOT NULL,
 "role_id" VARCHAR(50) NOT NULL,
 "status" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);

CREATE INDEX idx_user_role_u ON "user_role" ("user_id");
CREATE INDEX idx_user_role_r ON "user_role" ("role_id");


COMMENT ON TABLE "group" IS '';
COMMENT ON COLUMN "group"."group_id" IS '用户组code';
COMMENT ON COLUMN "group"."group_name" IS '';
COMMENT ON COLUMN "group"."remark" IS '';
COMMENT ON COLUMN "group"."status" IS '0 禁用  1启用';
COMMENT ON COLUMN "group"."created" IS '创建时间';
COMMENT ON COLUMN "group"."updated" IS '更新时间';
COMMENT ON TABLE "role" IS '';
COMMENT ON COLUMN "role"."role_id" IS '用户组code';
COMMENT ON COLUMN "role"."role_name" IS '';
COMMENT ON COLUMN "role"."remark" IS '';
COMMENT ON COLUMN "role"."status" IS '0 禁用  1启用';
COMMENT ON COLUMN "role"."created" IS '创建时间';
COMMENT ON COLUMN "role"."updated" IS '更新时间';
COMMENT ON TABLE "user_group" IS '';
COMMENT ON COLUMN "user_group"."user_id" IS '用户ID';
COMMENT ON COLUMN "user_group"."group_id" IS '用户组code';
COMMENT ON COLUMN "user_group"."status" IS '0 禁用  1启用';
COMMENT ON COLUMN "user_group"."created" IS '创建时间';
COMMENT ON COLUMN "user_group"."updated" IS '更新时间';
COMMENT ON TABLE "user_role" IS '';
COMMENT ON COLUMN "user_role"."id" IS '';
COMMENT ON COLUMN "user_role"."user_id" IS '用户ID';
COMMENT ON COLUMN "user_role"."role_id" IS '用户组code';
COMMENT ON COLUMN "user_role"."status" IS '0 禁用  1启用';
COMMENT ON COLUMN "user_role"."created" IS '创建时间';
COMMENT ON COLUMN "user_role"."updated" IS '更新时间';
