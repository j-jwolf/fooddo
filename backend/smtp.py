import os, sys
from datetime import datetime
import smtplib
from email.mime.text import MIMEText as text
from random import randint

sender = "fooddoapp@outlook.com"
password = "%k$0#XFm@X!jv4"
serverName = "smtp-mail.outlook.com"

# list of known email domains -- !!! MOVE TO FILE !!!
domains = {
	"outlook.com",
	"gmail.com",
	"hotmail.com",
	"icloud.com",
	"tutanota.com",
	"noctrl.edu"
}

fn = "verify.fdo"

def verifyEmails():
	emails = []
	for item in sys.argv:
		if("@" in item and len(item.split("@")) == 2 and item.split("@")[-1] in domains): emails.append(item)
	return emails
def generateCode():
	code = ""
	for i in range(8): code += str(randint(0, 10))
	return code
def sendEmail(reciever):
	code = None
	try:
		server = smtplib.SMTP(serverName, 587)
		server.ehlo()
		server.starttls()
		server.ehlo()
		server.login(sender, password)
		code = generateCode()
		subject = "Fooddo Email Verification"
		body = f"This is an official email from Fooddo and is part of the two step verification process\nYour verification code is {code}"
		msg = text(body, "plain", "utf-8")
		msg["Subject"] = subject
		msg["From"] = sender
		msg["To"] = reciever
		message = msg.as_string()
		server.sendmail(sender, reciever, message)
	except KeyboardInterrupt: sys.exit()
	except Exception as e: print(e)
	return code

if(len(sys.argv) == 1):
	# print to cerr or write error to file
	...
	sys.exit()

recipients = verifyEmails()
codeMap = {}
for recipient in recipients:
	code = sendEmail(recipient)
	codeMap[recipient] = str(code)
if(codeMap):
	with open(fn, "w") as file: file.write(str(codeMap))
