# test_that("panic doesn't crash R session", {
#   code <- r"(
# use savvy::savvy;
#
# #[allow(unreachable_code)]
# #[savvy]
# fn try_panic() -> savvy::Result<()> {
#    panic!("safe");
#    Ok(())
# }
# )"
#
#   savvy_override <- list(savvy = list(path = file.path(getwd(), "../../../")))
#   savvy::savvy_source(code, use_cache_dir = TRUE, dependencies = savvy_override)
#
#   expect_error(try_panic())
# })
