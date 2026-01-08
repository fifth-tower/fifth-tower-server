CREATE TABLE "dict" (
 "dict_code" VARCHAR(50) NOT NULL PRIMARY KEY,
 "dict_name" VARCHAR(50) NOT NULL,
 "parent_code" VARCHAR(50),
 "public" SMALLINT NOT NULL,
 "remark" VARCHAR(500),
 "status" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);


CREATE TABLE "dict_item" (
 "id" serial NOT NULL PRIMARY KEY,
 "dict_code" VARCHAR(50) NOT NULL,
 "item_name" VARCHAR(50) NOT NULL,
 "item_value" VARCHAR(500) NOT NULL,
 "public" SMALLINT NOT NULL,
 "remark" VARCHAR(500),
 "status" SMALLINT NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);


CREATE TABLE "user_vip_level" (
 "user_id" VARCHAR(50) NOT NULL PRIMARY KEY,
 "level_code" VARCHAR(50) NOT NULL,
 "status" SMALLINT NOT NULL,
 "expried" bigint NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);


COMMENT ON TABLE "dict" IS '';
COMMENT ON COLUMN "dict"."dict_code" IS 'id';
COMMENT ON COLUMN "dict"."dict_name" IS '用名呢称';
COMMENT ON COLUMN "dict"."parent_code" IS 'id';
COMMENT ON COLUMN "dict"."remark" IS '';
COMMENT ON COLUMN "dict"."status" IS '0 禁用  1启用';
COMMENT ON COLUMN "dict"."created" IS '创建时间';
COMMENT ON COLUMN "dict"."updated" IS '更新时间';
COMMENT ON TABLE "dict_item" IS '';
COMMENT ON COLUMN "dict_item"."id" IS '';
COMMENT ON COLUMN "dict_item"."dict_code" IS 'id';
COMMENT ON COLUMN "dict_item"."item_name" IS '项目名称';
COMMENT ON COLUMN "dict_item"."item_value" IS '项目值';
COMMENT ON COLUMN "dict_item"."public" IS '是否公共';
COMMENT ON COLUMN "dict_item"."remark" IS '';
COMMENT ON COLUMN "dict_item"."status" IS '0 禁用  1启用';
COMMENT ON COLUMN "dict_item"."created" IS '创建时间';
COMMENT ON COLUMN "dict_item"."updated" IS '更新时间';
COMMENT ON TABLE "user_vip_level" IS '';
COMMENT ON COLUMN "user_vip_level"."user_id" IS '用户ID';
COMMENT ON COLUMN "user_vip_level"."level_code" IS '对应dict_code:  VipLevel';
COMMENT ON COLUMN "user_vip_level"."status" IS '0 禁用  1启用';
COMMENT ON COLUMN "user_vip_level"."expried" IS '过期时间';
COMMENT ON COLUMN "user_vip_level"."created" IS '创建时间';
COMMENT ON COLUMN "user_vip_level"."updated" IS '更新时间';
