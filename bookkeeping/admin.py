from django.contrib import admin
from .models import Asset, Account, Transaction
# Register your models here.

admin.site.register(Asset)
admin.site.register(Account)
admin.site.register(Transaction)