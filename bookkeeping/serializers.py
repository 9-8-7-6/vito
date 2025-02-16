from rest_framework import serializers
from django.contrib.auth.models import User
from .models import Asset, Account, Transaction

class AccountSerializer(serializers.ModelSerializer):
    user = serializers.StringRelatedField()
    
    class Meta:
        model = Account
        fields = ['user', 'balance']


class AssetSerializer(serializers.ModelSerializer):
    user = serializers.StringRelatedField()

    class Meta:
        model = Asset
        fields = ['user', 'type', 'balance']

class TransactionSerializer(serializers.ModelSerializer):
    account = serializers.StringRelatedField(source='account.user')
    asset = serializers.StringRelatedField(source='asset.type')
    transaction_type = serializers.CharField(source='get_transaction_type_display')

    class Meta:
        model = Transaction
        fields = ['account', 'asset', 'transaction_type', 'amount']