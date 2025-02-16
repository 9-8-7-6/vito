from rest_framework import serializers
from django.contrib.auth.models import User
from .models import Asset, Account, Transaction, User

class UserSerializer(serializers.ModelSerializer):
    class Meta:
        model = User
        fields = ['name']


class AccountSerializer(serializers.ModelSerializer):    
    class Meta:
        model = Account
        fields = ['user_id', 'balance']


class AssetSerializer(serializers.ModelSerializer):
    class Meta:
        model = Asset
        fields = ['user', 'type', 'balance']

class TransactionSerializer(serializers.ModelSerializer):
    class Meta:
        model = Transaction
        fields = [
            'account', 'asset', 'transaction_type', 'from_account', 'to_account',  'amount'
        ]
