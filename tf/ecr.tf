resource "aws_ecrpublic_repository" "ecr_files" {
  provider = aws.us_east_1

  repository_name = "77beded4b02ff2bbc55625b157652fb0_files"
}