from rest_framework import serializers
from .models import Asset, Account, Transaction
from django.db import transaction

class AccountSerializer(serializers.ModelSerializer):    
    class Meta:
        model = Account
        fields = ['username', 'balance']

class AssetSerializer(serializers.ModelSerializer):
    account_username = serializers.CharField(write_only=True)

    class Meta:
        model = Asset
        fields = ['account_username', 'asset_type', 'balance']
        
    def create(self, validated_data):
        account_username = validated_data.pop('account_username')   # get account_username
        new_balance = validated_data.get("balance", 0)
        
        with transaction.atomic():
            account, created = Account.objects.get_or_create(
                username=account_username,
                defaults={"balance": new_balance}
            )
        
            if not created:
                account.balance += new_balance
                account.save()        
            
            asset = Asset.objects.create(account=account, **validated_data)

        return asset
        
    def update(self, instance, validated_data):
        new_balance = validated_data.get("balance", instance.balance)

        with transaction.atomic():
            account = instance.account
            account.balance += (new_balance - instance.balance)
            account.save()
            
            instance.balance = new_balance
            instance.save()
        
        return instance

class TransactionSerializer(serializers.ModelSerializer):
    class Meta:
        model = Transaction
        fields = [
            'asset', 'transaction_type', 'amount', 'from_account', 'to_account'
        ]

    def validate(self, data):
        transaction_type = data.get('transaction_type')
        from_account = data.get('from_account')
        to_account = data.get('to_account')
        asset = data.get('asset')
        amount = data.get('amount')

        if transaction_type == Transaction.TransactionType.TRANSFER:
            if not from_account or not to_account:
                raise serializers.ValidationError("Both from_account and to_account are required for a Transfer transaction.")
            if from_account == to_account:
                raise serializers.ValidationError("from_account and to_account cannot be the same account.")
            if from_account.balance < amount:
                raise serializers.ValidationError("Insufficient balance in from_account.")


        elif transaction_type == Transaction.TransactionType.EXPENSE:
            if to_account:
                raise serializers.ValidationError("to_account should only be set for Transfer transactions.")
            if data['from_account'].balance < amount:
                raise serializers.ValidationError("Insufficient balance for expense.")

        elif transaction_type == Transaction.TransactionType.INCOME:
            if to_account:
                raise serializers.ValidationError("to_account should only be set for Transfer transactions.")

        return data

    def create(self, validated_data):
        with transaction.atomic():
            amount = validated_data['amount']
            transaction_type = validated_data['transaction_type']
            from_account = validated_data.get('from_account')
            to_account = validated_data.get('to_account')

            from_account, _ = Account.objects.get_or_create(username=from_account.username, defaults={"balance": 0})

            # update balance
            if transaction_type == Transaction.TransactionType.INCOME:
                from_account.balance += amount

            elif transaction_type == Transaction.TransactionType.EXPENSE:
                from_account.balance -= amount

            elif transaction_type == Transaction.TransactionType.TRANSFER:
                to_account, _ = Account.objects.get_or_create(username=to_account.username, defaults={"balance": 0})
                to_account.balance += amount
                to_account.save()

                from_account.balance -= amount
                from_account.save()

            from_account.save()

            return super().create(validated_data)

    def update(self, instance, validated_data):
        with transaction.atomic():
            # restore the influence of old transaction first
            if instance.transaction_type == Transaction.TransactionType.INCOME:
                instance.from_account.balance -= instance.amount
            elif instance.transaction_type == Transaction.TransactionType.EXPENSE:
                instance.from_account.balance += instance.amount
            elif instance.transaction_type == Transaction.TransactionType.TRANSFER:
                if instance.to_account is None:
                    raise serializers.ValidationError("Transfer transaction must have a to_account.")
                instance.from_account.balance += instance.amount
                instance.to_account.balance -= instance.amount

            instance.from_account.save()
            if instance.transaction_type == Transaction.TransactionType.TRANSFER:
                instance.from_account.save()
                instance.to_account.save()
                
            for attr, value in validated_data.items():
                setattr(instance, attr, value)
                
            if instance.transaction_type == Transaction.TransactionType.INCOME:
                instance.from_account.balance += instance.amount
            elif instance.transaction_type == Transaction.TransactionType.EXPENSE:
                instance.from_account.balance -= instance.amount
            elif instance.transaction_type == Transaction.TransactionType.TRANSFER:
                instance.from_account.balance -= instance.amount
                instance.to_account.balance += instance.amount

            instance.save()
            return instance

    def delete(self, instance):
        with transaction.atomic():
            if instance.transaction_type == Transaction.TransactionType.INCOME:
                instance.from_account.balance -= instance.amount
            elif instance.transaction_type == Transaction.TransactionType.EXPENSE:
                instance.from_account.balance += instance.amount
            elif instance.transaction_type == Transaction.TransactionType.TRANSFER:
                instance.from_account.balance += instance.amount
                instance.to_account.balance -= instance.amount

            instance.from_account.save()
            if instance.transaction_type == Transaction.TransactionType.TRANSFER:
                instance.to_account.save()

            super(Transaction, instance).delete()
