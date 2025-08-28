import * as cdk from "aws-cdk-lib";
import { PochaMoeAPIStack } from "./stack";

const app = new cdk.App();
new PochaMoeAPIStack(app, "PochaMoeAPIStack", {
  env: { account: "575108959833", region: "us-east-1" },
});