from django.shortcuts import render
from django.db import transaction as db_transaction
from .serializers import AccountSerializer, AssetSerializer, TransactionSerializer
from .models import Account, Asset, Transaction
from rest_framework import viewsets, status
from rest_framework.response import Response

class AccountViewSet(viewsets.ModelViewSet):
    serializer_class = AccountSerializer
    queryset = Account.objects.all()

class AssetViewSet(viewsets.ModelViewSet):
    serializer_class = AssetSerializer
    queryset = Asset.objects.all()
    
    def destroy(self, request, *args, **kwargs):
        asset_instance = self.get_object()
        account = asset_instance.account
        balance_to_reduce = asset_instance.balance

        with db_transaction.atomic():
            account.balance -= balance_to_reduce
            account.save()
            
            asset_instance.delete()

        return Response({"message": "Asset deleted successfully."}, status=status.HTTP_204_NO_CONTENT)


class TransactionViewSet(viewsets.ModelViewSet):
    serializer_class = TransactionSerializer
    queryset = Transaction.objects.all()

    def create(self, request, *args, **kwargs):
        data = request.data
        if float(data.get("amount", 0)) <= 0:
            return Response({"error": "交易金額必須大於 0"}, status=status.HTTP_400_BAD_REQUEST)

        serializer = self.get_serializer(data=data)
        serializer.is_valid(raise_exception=True)

        with db_transaction.atomic():
            self.perform_create(serializer)
        
        return Response(serializer.data, status=status.HTTP_201_CREATED)

    def update(self, request, *args, **kwargs):
        transaction_instance = self.get_object()
        data = request.data

        serializer = self.get_serializer(transaction_instance, data=data, partial=True)
        serializer.is_valid(raise_exception=True)

        with db_transaction.atomic():
            serializer.save()

        return Response(serializer.data, status=status.HTTP_200_OK)

    def destroy(self, request, *args, **kwargs):
        transaction_instance = self.get_object()

        with db_transaction.atomic():
            super().delete(instance)

        return Response({"message": "Transaction deleted successfully."}, status=status.HTTP_204_NO_CONTENT)
