import json

class RestAPI:
    def __init__(self, database=None):
        self.__db = database
        if not self.__db:
            self.__db = {"users": []}

    def get(self, url, payload=None):
        if url == "/users":
            return self.__get_users(payload)

    def post(self, url, payload=None):
        if url == "/add":
            return self.__post_add_user(payload)
        elif url == "/iou":
            return self.__post_add_iou(payload)

    def __get_users(self, payload):
        response = self.__db
        if payload:
            response = {"users": []}
            for user_name in json.loads(payload)["users"]:
                response["users"].append(self.__get_user_from_db(user_name))
        response["users"].sort(key=lambda user: user["name"])
        return json.dumps(response)

    def __get_user_from_db(self, user_name):
        return next(user for user in self.__db["users"] if user["name"] == user_name)

    def __post_add_user(self, payload):
        user = {"name": json.loads(payload)["user"], "owes": {}, "owed_by": {}, "balance": 0.0}
        self.__db["users"].append(user)
        return json.dumps(user)

    def __post_add_iou(self, payload):
        iou = json.loads(payload)
        lender = self.__get_user_from_db(iou["lender"])
        borrower = self.__get_user_from_db(iou["borrower"])
        amount = iou["amount"]

        if borrower["name"] not in lender["owed_by"]:
            lender["owed_by"][borrower["name"]] = 0.0
        if borrower["name"] not in lender["owes"]:
            lender["owes"][borrower["name"]] = 0.0
        lender_owed = lender["owed_by"][borrower["name"]] - lender["owes"][borrower["name"]] + amount

        if lender_owed > 0:
            lender["owed_by"][borrower["name"]] = lender_owed
            lender["owes"].pop(borrower["name"], None)
            borrower["owes"][lender["name"]] = lender_owed
            borrower["owed_by"].pop(lender["name"], None)

        elif lender_owed < 0:
            lender["owes"][borrower["name"]] = -lender_owed
            lender["owed_by"].pop(borrower["name"], None)
            borrower["owed_by"][lender["name"]] = -lender_owed
            borrower["owes"].pop(lender["name"], None)

        else:
            lender["owed_by"].pop(borrower["name"], None)
            lender["owes"].pop(borrower["name"], None)
            borrower["owed_by"].pop(lender["name"], None)
            borrower["owes"].pop(lender["name"], None)

        lender["balance"] += amount
        borrower["balance"] -= amount

        return self.__get_users(json.dumps({"users": [lender["name"], borrower["name"]]}))
