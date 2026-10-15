<h1 align="center">Katalon Petstore API Automation Testing (katalon-petstore-api)</h1>

![Katalon Studio](https://img.shields.io/badge/Tool-Katalon%20Studio%20v11.x-green)
![App](https://img.shields.io/badge/Api-petstore.swagger.io-orange)

 ## 📌 Project Overview </h3>

This repository contains an automated API testing suite built with Katalon Studio to test the endpoints of the public Swagger Petstore API (https://petstore.swagger.io/v2).

The project covers CRUD operations (Create, Read, Update, Delete) on the /pet resource. It is structured using modular Test Cases, Object Repository dynamic variables, and custom Groovy scripts to validate response status codes, JSON schema structures, and response body payloads for both positive and negative testing scenarios.

## 🛠️ Tech Stack & Tools

Testing Tool: Katalon Studio v11.x
Programming Language: Groovy / Java
Target API: Swagger Petstore REST API (v2)
Data Format: JSON
Version Control: Git & GitHub

## 📂 Project Structure

```text
|   .classpath
|   .gitignore
|   .project
|   ApiAfterOffice.prj
|   build.gradle
|   console.properties
|
+---.settings
|       io.cucumber.eclipse.java.prefs
|       org.eclipse.core.resources.prefs
|       org.eclipse.jdt.core.prefs
|       org.eclipse.jdt.groovy.core.prefs
|
+---bin
|   |   .project
|   |
|   +---groovy
|   +---keyword
|   +---lib
|   |   |   CustomKeywords.class
|   |   |   S1015_0.class
|   |   |
|   |   \---internal
|   |           GlobalVariable.class
|   |
|   \---listener
+---Checkpoints
+---Data
|       deleteSchema.json
|       getSchema.json
|       getSchemaIdNotFound.json
|       PostSchema.json
|       postSchemaNotFound.json
|       putSchema.json
|       putSchemaNotFound.json
|
+---Data Files
+---Drivers
+---Get
+---Include
|   +---config
|   |       log.properties
|   |
|   +---features
|   \---scripts
|       \---groovy
+---Keywords
+---Libs
|   |   CustomKeywords.groovy
|   |   S1015_0.groovy
|   |
|   \---internal
|           GlobalVariable.groovy
|
+---Object Repository
|   \---petstore.swagger
|       \---Pet
|           +---Delete
|           |       delete-pet-deletePet.rs
|           |       delete-pet-notFound.rs
|           |
|           +---Get
|           |       get-pet-findPet.rs
|           |       get-pet-notFound.rs
|           |
|           +---Post
|           |       post-pet-addNewPet.rs
|           |       post-pet-serverError.rs
|           |
|           \---Put
|                   put-pet-existing.rs
|                   put-pet-serverError.rs
|
+---Plugins
+---Profiles
|       default.glbl
|
+---Reports
|   +---20261015_202112
|   |   |   plan.jsonl
|   |   |
|   |   +---.metadata
|   |   |   \---E2e
|   |   |       \---addPetId-getFind-deleteNotFound
|   |   \---E2e
|   |       \---addPetId-getFind-deleteNotFound
|   |           |   addPetId-getFind-deleteNotFound.html
|   |           |
|   |           \---20261015_202112
|   |               |   20261015_202112.csv
|   |               |   20261015_202112.html
|   |               |   console0.log
|   |               |   execution.properties
|   |               |   execution.uuid
|   |               |   execution0.log
|   |               |   execution0.log.lck
|   |               |   JUnit_Report.xml
|   |               |   testCaseBinding
|   |               |   tsc_id.txt
|   |               |
|   |               \---requests
|   |                   \---main
|   |                           delete-pet-notFound_2.har
|   |                           post-pet-addNewPet_0.har
|   |                           post-pet-addNewPet_1.har
|   |
|   +---20261015_203216
|   |   |   plan.jsonl
|   |   |
|   |   +---.metadata
|   |   |   \---E2e
|   |   |       \---addPetId-getFind-putExist-deletePet
|   |   \---E2e
|   |       \---addPetId-getFind-putExist-deletePet
|   |           |   addPetId-getFind-putExist-deletePet.html
|   |           |
|   |           \---20261015_203217
|   |               |   20261015_203217.csv
|   |               |   20261015_203217.html
|   |               |   console0.log
|   |               |   execution.properties
|   |               |   execution.uuid
|   |               |   execution0.log
|   |               |   JUnit_Report.xml
|   |               |   testCaseBinding
|   |               |   tsc_id.txt
|   |               |
|   |               \---requests
|   |                   \---main
|   |                           delete-pe...letePet_3.har
|   |                           post-pet-addNewPet_0.har
|   |                           post-pet-addNewPet_1.har
|   |                           put-pet-existing_2.har
|   |
|   \---Self-healing
|           broken-test-objects-v2.json
|
+---Scripts
|   +---Case Pet
|   |   +---Delete
|   |   |   +---Delete-Pet-DeletePet
|   |   |   |       Script1791918517370.groovy
|   |   |   |
|   |   |   \---Delete-pet-NotFound
|   |   |           Script1792056986450.groovy
|   |   |
|   |   +---Delete-Pet-DeletePet
|   |   +---Ger
|   |   +---Get
|   |   |   +---Get-Pet-FindPet
|   |   |   |       Script1791918485956.groovy
|   |   |   |
|   |   |   \---Get-Pet-PetNotFound
|   |   |           Script1791964808552.groovy
|   |   |
|   |   +---Get-Pet-FindPet
|   |   +---Post
|   |   |   +---Post-Pet-AddNewPet
|   |   |   |       Script1791918497965.groovy
|   |   |   |
|   |   |   \---Post-Pet-ServerError
|   |   |           Script1791996224669.groovy
|   |   |
|   |   +---Post-Pet-AddNewPet
|   |   +---Put
|   |   |   +---Put-Pet-Existing
|   |   |   |       Script1791918531362.groovy
|   |   |   |
|   |   |   \---Put-Pet-ServerError
|   |   |           Script1792028620385.groovy
|   |   |
|   |   \---Put-Pet-Existing
|   \---test
+---settings
|   +---external
|   |       com.kms.katalon.composer.execution.settings.properties
|   |       com.kms.katalon.core.db.DatabaseSettings.properties
|   |       com.kms.katalon.integration.azure.properties
|   |
|   \---internal
|           com.katalon.cucumber.properties
|           com.kms.katalon.composer.project.properties
|           com.kms.katalon.composer.testcase.properties
|           com.kms.katalon.composer.testcase.settings.properties
|           com.kms.katalon.core.webservice.properties
|           com.kms.katalon.execution.properties
|           com.kms.katalon.execution.webservice.properties
|           com.kms.katalon.execution.webui.properties
|           com.kms.katalon.execution.windows.properties
|           com.kms.katalon.integration.qtest.properties
|
+---Test Cases
|   \---Case Pet
|       +---Delete
|       |       Delete-Pet-DeletePet.tc
|       |       Delete-pet-NotFound.tc
|       |
|       +---Get
|       |       .meta
|       |       Get-Pet-FindPet.tc
|       |       Get-Pet-PetNotFound.tc
|       |
|       +---Post
|       |       Post-Pet-AddNewPet.tc
|       |       Post-Pet-ServerError.tc
|       |
|       \---Put
|               Put-Pet-Existing.tc
|               Put-Pet-ServerError.tc
|
+---Test Listeners
\---Test Suites
    \---E2e
            addPetId-getFind-deleteNotFound.groovy
            addPetId-getFind-deleteNotFound.ts
            addPetId-getFind-putExist-deletePet.groovy
            addPetId-getFind-putExist-deletePet.ts

```
## 📊 Test Suite & Test Coverage

| Nama Test Suite | Method | Expected Status | Status |
| --- | --- | --- | --- |
| Add Pet, Get Find Pet, Delete Not Found | E2e | 200 OK |🟢 PASSED |
| Add Pet, Get Find Pet, Put Existing Pet , Delete Pet | E2e | 200 OK |🟢 PASSED |
| Add Pet, Get Find Pet, Put Server Error | E2e | 200 OK | 🟢 PASSED |

## 🧪 Test Suite Configuration

All individual test cases are aggregated into the main execution suite:
* Test Suite Path: Test Suites/test
* Execution Mode: Sequential / Parallel
* Execution Details:
1. Creates a new pet (POST) and captures the dynamic petId.
2. Fetches pet data (GET) and asserts JSON key-value response structures.
3. Updates pet attributes (PUT) and verifies updated fields.
4. Tests boundary & error scenarios (GET 404, PUT 500, POST 500).
5. Cleans up created records (DELETE).

## 🚀 How to Run the Tests

Prerequisites
1. Install Katalon Studio (Version 8.x or 11.x recommended).
2. Active internet connection to reach https://petstore.swagger.io.

Running via Katalon Studio GUI
1. Clone the Repository
2. Open Project:
Launch Katalon Studio -> File -> Open Project -> Select the katalon-petstore-api folder.
3. Run Individual Test Case:
    *  Navigate to Test Cases/Case Pet/ in the Test Explorer.
    *  Open desired test case (e.g., Get-Pet-FindPet).
    *  Click the Run button (Green Play Icon) on the top toolbar.
4. Run Complete Test Suite:
    * Navigate to Test Suites/test.
    * Click Run to execute all test cases in sequence.

## 📝 Key Features & Implementation Highlights

* Dynamic Test Data: Uses System.currentTimeMillis() and BigInteger in Groovy scripts to generate random, collision-free IDs and names during execution.
* Global Variables Sharing: Pass dynamic variables across multiple API test cases sequentially using GlobalVariable.
* Assertions & Validations:
 * HTTP Response Status Code Checks (200, 404, 500).
 * Field-level verification (WS.verifyElementPropertyValue).
 * Response Body parsing using groovy.json.JsonSlurper.

 ## ⚠️ Notes
 
* Secrets & Credentials: No passwords, private API keys, or tokens are committed to this repository.
* Public Server Behavior: petstore.swagger.io is a public demo environment, so mock/overflow tests (like status 500) are calibrated to simulate backend integer limit handling.
