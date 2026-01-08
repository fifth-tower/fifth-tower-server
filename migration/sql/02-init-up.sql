CREATE TABLE "social_report" (
 "id" serial NOT NULL PRIMARY KEY,
 "target_id" VARCHAR(50) NOT NULL,
 "target_type" SMALLINT NOT NULL,
 "like_count" bigint NOT NULL,
 "follow_count" bigint NOT NULL,
 "favorite_count" bigint NOT NULL,
 "comment_count" bigint NOT NULL,
 "score_count" bigint NOT NULL,
 "score_total" bigint NOT NULL,
 "created" bigint NOT NULL,
 "updated" bigint
);


COMMENT ON TABLE "social_report" IS '社交属性报表';
COMMENT ON COLUMN "social_report"."id" IS '应用ID';
COMMENT ON COLUMN "social_report"."target_id" IS '被评分对象ID';
COMMENT ON COLUMN "social_report"."target_type" IS '被评分对象类型';
COMMENT ON COLUMN "social_report"."like_count" IS '点赞人数';
COMMENT ON COLUMN "social_report"."follow_count" IS '关注人数';
COMMENT ON COLUMN "social_report"."favorite_count" IS '收藏人数';
COMMENT ON COLUMN "social_report"."comment_count" IS '评论人数';
COMMENT ON COLUMN "social_report"."score_count" IS '评分人数';
COMMENT ON COLUMN "social_report"."score_total" IS '总评分';
COMMENT ON COLUMN "social_report"."created" IS '创建时间';
COMMENT ON COLUMN "social_report"."updated" IS '更新时间';
