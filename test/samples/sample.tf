resource "aws_instance" "example" {
ami="ami-12345"
instance_type="t2.micro"
tags={Name="test"
Environment="dev"}
}
