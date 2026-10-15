import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.llm.keyword.LlmKeywords as LLM
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper

String randomBigNum = "11" + System.currentTimeMillis() + (100000 + new Random().nextInt(900000))
def petIdNotFound = new BigInteger(randomBigNum)
def userRequest = WS.sendRequest(findTestObject('Object Repository/petstore.swagger/Pet/Put/put-pet-serverError',[
	'petIdNotFound': petIdNotFound
	]))
def jsonRequest = new JsonSlurper().parseText(userRequest.getResponseText())

WS.verifyResponseStatusCode(userRequest, 500)
assert jsonRequest.code != null
assert jsonRequest.message != null
assert jsonRequest.message instanceof String

assert jsonRequest.code == 500
assert jsonRequest.type != null && jsonRequest.type instanceof String
assert jsonRequest.message != null && !jsonRequest.message.isEmpty()
println("Response status Code: " + jsonRequest.code)

